use crate::{
    mocks::{user_model_mock::{get_mock_user_model, MockUserModelParams, MockUserModelUpdate}, 
    sanitizer_user_input_mock::*,},
};
use authentication_gRPC::{
    controllers::authentication::authentication_controller::{
        UpdateParams, AuthenticationController, UserController, UserViewArg
    },
    error::*,
    models::authentication::authentication_model::{UserModelUpdateParams, UserModelUpdateReturn},
    security::jwt::{JWTAuthenticateToken, UserToken}
};

const FAKE_USER_ID: &str = "user_id";
const FAKE_USERNAME: &str = "username";
const FAKE_EMAIL: &str = "test@controller.com";
const FAKE_PASSWORD: &str = "password";
const FAKE_JWT_TOKEN: &str = "fake_jwt_token";

const SANITIZED_USERNAME: &str = "username_sanitized";
const SANITIZED_EMAIL: &str = "sanitized@email.com";
const SANITIZED_PASSWORD: &str = "password_sanitized";

#[tokio::test]
async fn test_update() {
    let expectation_of_sanitize_username = MockUserInputSanitizeUsername {
        calls: 1,
        param_username_with: FAKE_USERNAME.to_string(),
        fn_returning: |_| Ok(SANITIZED_USERNAME.to_string()),
    };

    let expectation_of_sanitize_email = MockUserInputSanitizeEmail {
        calls: 1,
        param_email_with: FAKE_EMAIL.to_string(),
        fn_returning: |_| Ok(SANITIZED_EMAIL.to_string()),
    };

    let expectation_of_sanitize_password = MockUserInputSanitizePassword {
        calls: 1,
        param_password_with: FAKE_PASSWORD.to_string(),
        fn_returning: |_| Ok(SANITIZED_PASSWORD.to_string()),
    };

    let user_model_params = UserModelUpdateParams {
        username: Some(SANITIZED_USERNAME.to_string()),
        email: Some(SANITIZED_EMAIL.to_string()),
        password: Some(SANITIZED_PASSWORD.to_string()),
    };

    let expectations_of_the_methods_that_will_be_used = MockUserModelParams {
        update: Some(MockUserModelUpdate {
            calls: 1,
            param_id_with: FAKE_USER_ID.to_string(),
            param_user_with: user_model_params,
            fn_returning: mock_user_model_update,
        }),
        ..Default::default()
    };

    let controller = UserController {
        model: get_mock_user_model(expectations_of_the_methods_that_will_be_used),
        sanitize_user: get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
            username: Some(expectation_of_sanitize_username),
            email: Some(expectation_of_sanitize_email),
            password: Some(expectation_of_sanitize_password),
        }),
        jwt_encode: mock_jwt_encode,
        jwt_decode: mock_jwt_decode,
    };
    let req = UpdateParams {
        username: Some(FAKE_USERNAME.to_string()),
        email: Some(FAKE_EMAIL.to_string()),
        password: Some(FAKE_PASSWORD.to_string()),
    };

    let MockViewReturn { user } = controller
        .update(FAKE_JWT_TOKEN.to_string(), req, view_mock)
        .await
        .unwrap();

    assert_eq!(user.id, FAKE_USER_ID);
    assert_eq!(user.username, SANITIZED_USERNAME);
    assert_eq!(user.email, SANITIZED_EMAIL);
}

struct MockViewReturn {
    user: UserViewArg,
}

fn view_mock(user: UserViewArg) -> MockViewReturn {
    MockViewReturn { user }
}

fn mock_user_model_update(
    id: String,
    user: UserModelUpdateParams,
) -> Result<UserModelUpdateReturn, AppError> {
    if id != FAKE_USER_ID {
        return Err(AppError::new(
            Code::NotFound,
            "not found the user with the given ID",
        ));
    }

    Ok(UserModelUpdateReturn {
        id,
        username: user.username.unwrap_or(FAKE_USERNAME.to_string()),
        email: user.email.unwrap_or(FAKE_EMAIL.to_string()),
    })
}

fn mock_jwt_encode(user: UserToken) -> Result<String, AppError> {
    let UserToken {
        id,
        username,
        email,
    } = user;
    if id != FAKE_USER_ID || username != SANITIZED_USERNAME || email != SANITIZED_EMAIL {
        panic!("received invalid wrong expected params")
    }

    Ok(FAKE_JWT_TOKEN.to_string())
}

fn mock_jwt_decode(token: &str) -> Result<JWTAuthenticateToken, AppError> {
    if token != FAKE_JWT_TOKEN {
        return Err(AppError::new(Code::PermissionDenied, "Invalid token"));
    }

    Ok(JWTAuthenticateToken {
        sub: FAKE_USER_ID.to_string(),
        user: UserToken {
            id: FAKE_USER_ID.to_string(),
            username: FAKE_USERNAME.to_string(),
            email: FAKE_EMAIL.to_string(),
        },
        exp: 99999999,
    })
}
