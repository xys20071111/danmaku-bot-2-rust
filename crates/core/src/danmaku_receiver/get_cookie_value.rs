pub fn get_cookie_value(cookies: &String, name: &str) -> Result<String, ()> {
    let cookies: Vec<&str> = cookies.split(";").collect();
    for cookie in cookies.iter() {
        if cookie.starts_with(name) {
            let cookie: Vec<&str> = cookie.split("=").collect();
            return Ok(String::from(cookie[1]));
        }
    }
    Err(())
}
