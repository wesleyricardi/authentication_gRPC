use authentication_gRPC::{
    controllers::authentication::authentication_controller::{
        AuthenticationController, UserController,
    },
    error::{AppError, Code},
    models::authentication::authentication_model::{CodeType, UserModelRecoverUserDataReturn},
    security::jwt::JWTAuthenticateToken,
};

use crate::mocks::{
    sanitizer_user_input_mock::{get_mock_user_input_sanitizer, MockUserInputSanitizeParams},
    user_model_mock::{
        get_mock_user_model, MockUserModelCreateUserCode, MockUserModelParams,
        MockUserModelRecoverUserData,
    },
};

const FAKE_USER_ID: &str = "user_id";
const FAKE_EMAIL: &str = "test@controller.com";
const FAKE_USERNAME: &str = "username";
const FAKE_USER_CODE: &str = "000001";
const FAKE_JWT_TOKEN: &str = "fake_jwt_token";

#[tokio::test]
async fn test_authentication() {
    let expectations_of_the_methods_that_will_be_used = MockUserModelParams {
        recover_user_data: Some(MockUserModelRecoverUserData {
            calls: 1,
            param_id_with: FAKE_USER_ID.to_string(),
            fn_returning: mock_user_model_recover,
        }),

        create_user_code: Some(MockUserModelCreateUserCode {
            calls: 1,
            param_user_id_with: FAKE_USER_ID.to_string(),
            param_code_type_with: CodeType::Activation,
            fn_returning: mock_user_model_create_code,
        }),
        ..Default::default()
    };

    let controller = UserController {
        model: get_mock_user_model(expectations_of_the_methods_that_will_be_used),
        sanitize_user: get_mock_user_input_sanitizer(MockUserInputSanitizeParams {
            ..Default::default()
        }),
        send_email: |to, subject, body| {
            assert_eq!(to, FAKE_EMAIL);
            assert_eq!(subject, "activation code");
            assert_eq!(
                body,
                format!("<div>The activation code is {}</div>", FAKE_USER_CODE)
            );

            Ok(String::from("Send email successfully"))
        },
        jwt_encode: |_, _, _| panic!("Should not be called"),
        jwt_decode: mock_jwt_decode,
    };

    let response = controller
        .send_activation_code(FAKE_JWT_TOKEN.to_string(), view_mock)
        .await
        .unwrap();

    assert_eq!(response, "Code send successufully");
}

fn view_mock(message: String) -> String {
    message
}

fn mock_user_model_recover(id: String) -> Result<UserModelRecoverUserDataReturn, AppError> {
    if id != FAKE_USER_ID {
        return Err(AppError::new(
            Code::NotFound,
            "not found the user with the given ID",
        ));
    }

    Ok(UserModelRecoverUserDataReturn {
        username: FAKE_USERNAME.to_string(),
        email: FAKE_EMAIL.to_string(),
        activated: false,
        blocked: false,
    })
}

fn mock_user_model_create_code(_: String, _: CodeType) -> Result<String, AppError> {
    Ok(FAKE_USER_CODE.to_string())
}

fn mock_jwt_decode(token: &str) -> Result<JWTAuthenticateToken, AppError> {
    if token != FAKE_JWT_TOKEN {
        return Err(AppError::new(Code::PermissionDenied, "Invalid token"));
    }

    Ok(JWTAuthenticateToken {
        sub: FAKE_USER_ID.to_string(),
        activated: false,
        blocked: false,
        exp: 99999999,
    })
}
