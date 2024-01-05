use std::process::Command;

///
/// # Represent a notification
///
pub struct Notification {
    icon: String,
    summary: String,
    body: String,
    app: String,
    timeout: i32,
}

///
/// # Build a notification
///
impl Default for Notification {
    fn default() -> Self {
        Self {
            icon: String::from("dialog-information"),
            summary: String::new(),
            body: String::new(),
            app: String::new(),
            timeout: 5000,
        }
    }
}

impl Notification {
    ///
    /// Notification constructor
    ///
    #[must_use]
    pub fn new() -> Self {
        Self {
            icon: String::from("dialog-information"),
            summary: String::new(),
            body: String::new(),
            app: String::new(),
            timeout: 5000,
        }
    }

    ///
    /// # Set notification summary
    ///
    /// - `text` The summary content
    ///
    pub fn summary(&mut self, text: &str) -> &mut Self {
        self.summary.clear();
        self.summary = text.to_string();
        self
    }

    ///
    /// # Set notification timeout
    ///
    /// - `time` The expire time in ms default 5000
    ///
    pub fn timeout(&mut self, time: i32) -> &mut Self {
        self.timeout = time;
        self
    }

    ///
    /// # Set notification body
    ///
    /// - `text` The body content
    ///
    pub fn body(&mut self, text: &str) -> &mut Self {
        self.body.clear();
        self.body = text.to_string();
        self
    }

    ///
    /// # Set the app name
    ///
    /// - `name` The name of application
    ///
    pub fn app(&mut self, name: &str) -> &mut Self {
        self.app.clear();
        self.app = name.to_string();
        self
    }

    ///
    /// # Set the icon
    ///
    /// - `name` The name of the icon default dialog-information
    ///
    pub fn icon(&mut self, name: &str) -> &mut Self {
        self.icon.clear();
        self.icon = name.to_string();
        self
    }

    /// # Panics
    ///
    /// Will panic if notify-send is not founded
    ///
    pub fn send(&mut self) -> bool {
        if self.app.is_empty() {
            Command::new("notify-send")
                .arg("-t")
                .arg(self.timeout.to_string().as_str())
                .arg("-i")
                .arg(self.icon.as_str())
                .arg(self.summary.as_str())
                .arg(self.body.as_str())
                .spawn()
                .expect("Fail to launch notify-send")
                .wait()
                .expect("msg")
                .success()
        } else {
            Command::new("notify-send")
                .arg("-t")
                .arg(self.timeout.to_string().as_str())
                .arg("-a")
                .arg(self.app.as_str())
                .arg("-i")
                .arg(self.icon.as_str())
                .arg(self.summary.as_str())
                .arg(self.body.as_str())
                .spawn()
                .expect("Fail to launch notify-send")
                .wait()
                .expect("msg")
                .success()
        }
    }
}
