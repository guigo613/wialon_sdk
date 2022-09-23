use std::io::{
    Error as IoError,
    ErrorKind
};
use super::{
    external::DeserializeOwned,
    *
};

#[derive(Debug)]
pub struct Wialon {
    user: String,
    token: String,
    sid: Option<String>
}

impl Wialon {
    pub fn new(user: &str, pass: &str) -> GenResult<Self> {
        Self::builder(user, pass).build()
    }

    pub fn builder<'a>(user: &'a str, pass: &'a str) -> BuilderWialon<'a> {
        BuilderWialon::new(user, pass)
    }

    fn request<T>(&self, params: &str) -> GenResult<T>
        where T: WialonResponse
    {
        let response = reqwest::get(self._request(T::svc(), params))?.text()?;

        Self::_deseliarize(&response)
    }

    fn _request(&self, svc: &str, params: &str) -> String {
        format!("{}?{}&{}", URL, SVC.replace("<svc>", svc), PARAMS.replace("<params>", params)) + &if let Some(s) = self.sid.as_ref() {
            format!("&{}", SID.replace("<sid>", s))
        } else { Default::default() }
    }

    fn _deseliarize<'a, T>(json: &str) -> GenResult<T>
        where T: DeserializeOwned
    {
        match serde_json::from_str(json) {
            Ok(json) => Ok(json),
            Err(err) => {
                eprintln!("{:?}", err);
                Err(serde_json::from_str::<Error>(json)?)?
            }
        }
    }

    pub fn connect(&mut self, flags: Option<u32>) -> GenResult<()> {
        let params_login = params::Login::new(self.token.clone(), flags, Some(self.user.clone()));
        let login: Login = self.request(&serde_json::to_string(&params_login)?)?;

        self.sid = Some(login.eid);

        println!("Login realizado com sucesso!\r\n\r\n{:?}", self.sid);

        Ok(())
    }

    pub fn list_token(&self) -> GenResult<Vec<TokenList>> {
        let list = self.request(params::VOID)?;
        
        Ok(list)
    }

    pub fn search_item(&self, search_params: &params::SearchItem) -> GenResult<SearchItem> {
        let list = self.request(&serde_json::to_string(search_params)?)?;

        Ok(list)
    }

    pub fn search_items(&self, search_params: &params::SearchItems) -> GenResult<SearchItems> {
        let list = self.request(&serde_json::to_string(search_params)?)?;

        Ok(list)
    }

    pub fn update_data_flags(&self, data_flags: &ParamsDataFlags) -> GenResult<Vec<UpdateDataFlags>> {
        let list = self.request::<Vec<UpdateDataFlags>>(&serde_json::to_string(data_flags)?)?;

        Ok(list)
    }

    pub fn events(&self) -> GenResult<AvlEvts> {
        let response = if let Some(sid) = self.sid.as_ref() {
            reqwest::get(format!("{}?{}", AVL, SID.replace("<sid>", sid)))?.text()?
        } else {
            Err(Error { error: 1 })?
        };

        Self::_deseliarize(&response)
    }

    pub fn disconnect(&mut self) -> GenResult<Logout> {
        let logout = self.request(params::VOID)?;

        println!("Disconnect...");

        Ok(logout)
    }
}

impl Drop for Wialon {
    fn drop(&mut self) {
        let _ = self.disconnect();
    }
}

pub struct BuilderWialon<'a> {
    response_type: &'a str,
    wialon_sdk_url: &'a str,
    success_uri: &'a str,
    client_id: &'a str,
    redirect_uri: &'a str,
    access_type: &'a str,
    activation_time: &'a str,
    duration: &'a str,
    flags: &'a str,
    login: &'a str,
    passw: &'a str
}

impl<'a> BuilderWialon<'a> {
    fn new(user: &'a str, pass: &'a str) -> Self {
        Self {
            response_type:   "token",
            wialon_sdk_url:  "https://hst-api.wialon.com",
            success_uri:     "",
            client_id:       "Rust Wialon",
            redirect_uri:    "http://hosting.wialon.com/login.html",
            access_type:     "-1",
            activation_time: "0",
            duration:        "100",
            flags:           "0",
            login:           user,
            passw:           pass
        }
    }

    pub fn set_response_type(mut self, v: &'a str) -> Self {
        self.response_type = v;
        self
    }

    pub fn set_wialon_sdk_url(mut self, v: &'a str) -> Self {
        self.wialon_sdk_url = v;
        self
    }

    pub fn set_success_uri(mut self, v: &'a str) -> Self {
        self.success_uri = v;
        self
    }

    pub fn set_client_id(mut self, v: &'a str) -> Self {
        self.client_id = v;
        self
    }

    pub fn set_redirect_uri(mut self, v: &'a str) -> Self {
        self.redirect_uri = v;
        self
    }

    pub fn set_access_type(mut self, v: &'a str) -> Self {
        self.access_type = v;
        self
    }

    pub fn set_activation_time(mut self, v: &'a str) -> Self {
        self.activation_time = v;
        self
    }

    pub fn set_duration(mut self, v: &'a str) -> Self {
        self.duration = v;
        self
    }

    pub fn set_flags(mut self, v: &'a str) -> Self {
        self.flags = v;
        self
    }

    pub fn build(self) -> GenResult<Wialon> {
        let get = reqwest::get(LOGIN.replace("<user>", self.login))?;
        let body = get.text()?;
        let client = reqwest::Client::new();

        let re = Regex::new(r#"<input(\s+type="\S*?"|\s+name="sign"){2}\s+value="(?P<sign>\S+)">"#)?;
        let sign = re.captures(&body).unwrap();

        let params = [
            ("response_type",   self.response_type),
            ("wialon_sdk_url",  self.wialon_sdk_url),
            ("success_uri",     self.success_uri),
            ("client_id",       self.client_id),
            ("redirect_uri",    self.redirect_uri),
            ("access_type",     self.access_type),
            ("activation_time", self.activation_time),
            ("duration",        self.duration),
            ("flags",           self.flags),
            ("sign",            sign.name("sign").unwrap().as_str()),
            ("login",           self.login),
            ("passw",           self.passw)
        ];
        
        let response = client.post(OAUTH.replace("<user>", self.login))
            .form(&params).send()?;

        let token = response.url().query_pairs().find(|x| { x.0 == "access_token" }).ok_or(IoError::from(ErrorKind::InvalidData))?.1;

        Ok(Wialon {
            user: self.login.into(),
            token: token.into(),
            sid: None
        })
    }
}