use curl::http;
use serialize::{json, Encodable};

pub struct Slack {
    domain : &'static str,
    token  : &'static str
}

impl Slack {
    pub fn send(&self, payload: Payload) {
        let url = format!("https://{}.slack.com/services/hooks/incoming-webhook?token={}",self.domain, self.token);
        println!("url = {}", url);
        println!("sending payload, {}", payload);
        let resp = http::handle()
          .post(url, &json::encode(&payload))
          .exec().unwrap();
        println!("{}",resp);
    }
}

#[deriving(Encodable, Show)]
pub struct Payload {
    channel      : &'static str,
    text         : &'static str,
    username     : Option<&'static str>,
    icon_url     : Option<&'static str>,
    icon_emoji   : Option<&'static str>,
    attachments  : Option<Attachments>,
    unfurl_links : Option<u8>,
    link_names   : Option<u8>
}

#[deriving(Encodable, Show)]
pub struct Attachments {
    fallback : &'static str,
    text     : Option<&'static str>,
    pretext  : Option<&'static str>,
    color    : &'static str,
    fields   : Vec<Attachment>
}

#[deriving(Encodable, Show)]
pub struct Attachment {
    title : &'static str,
    value : &'static str,
    short : Option<bool>
}
