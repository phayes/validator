#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::Validate;


#[cfg(feature = "phone")]
#[test]
fn can_validate_phone_ok() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(phone)]
        val: String,
    }

    let s = TestStruct {
        val: "+14152370800".to_string(),
    };

    assert!(s.validate().is_ok());
}

#[cfg(feature = "phone")]
#[test]
fn bad_phone_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(phone)]
        val: String,
    }

    let s = TestStruct {
        val: "bob".to_string(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "phone");
}

#[cfg(feature = "phone")]
#[test]
fn can_specify_code_for_phone() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(phone(code = "oops"))]
        val: String,
    }
    let s = TestStruct {
        val: "bob".to_string(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
    assert_eq!(errs["val"][0].params["value"], "bob");
}

#[cfg(feature = "phone")]
#[test]
fn can_specify_message_for_phone() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(phone(message = "oops"))]
        val: String,
    }
    let s = TestStruct {
        val: "bob".to_string(),
    };
    let res = s.validate();
    assert!(res.is_err());
    let errs = res.unwrap_err().inner();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}
