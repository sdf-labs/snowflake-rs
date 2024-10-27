// modeled after https://github.com/snowflakedb/snowflake-connector-python/blob/c9650701e19b597302ca25525e48406ba0b47425/src/snowflake/connector/util_text.py#L259
pub fn parse_account(account: &str) -> String {
    let url_parts: Vec<&str> = account.split('.').collect();
    let parsed_account = if url_parts.len() > 1 {
        if url_parts[1] == "GLOBAL" {
            // Remove external ID from account
            if let Some(pos_dash) = url_parts[0].rfind('-') {
                &url_parts[0][..pos_dash]
            } else {
                url_parts[0]
            }
        } else {
            // Remove region subdomain or other subdomains
            url_parts[0]
        }
    } else {
        account
    };

    parsed_account.to_string()
}

mod tests {

    #[test]
    fn test_parse_account() {
        use super::parse_account;
        let cases = [
            ("S3TESTACCOUNT.GLOBAL", "S3TESTACCOUNT.GLOBAL"),
            ("ACCT-WITH-DASHES", "ACCT-WITH-DASHES"),
            ("TESTACCOUNT.EXTRA", "TESTACCOUNT"),
            ("TESTACCOUNT-USER.GLOBAL", "TESTACCOUNT"),
            ("NORMALACCOUNT", "NORMALACCOUNT"),
            ("ACCOUNT.US-EAST-2.AWS", "ACCOUNT"),
        ];
        for (input, expected) in cases {
            assert_eq!(parse_account(input), expected);
        }
    }
}
