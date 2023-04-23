use std::env;

pub mod cpu_logger;

pub trait FromEnvList {
    fn from_env_list(env_list: &[String]) -> Self;
}

pub struct Builder {
    list: Box<[String]>,
}

impl Builder {
    pub fn build<T>(self) -> T
    where
        T: FromEnvList,
    {
        T::from_env_list(&self.list)
    }
}

pub trait DebugLogger: Default + FromEnvList {
    fn env(env_var: &str) -> Builder {
        let env_contents = env::var(env_var).unwrap_or("".into());

        Builder {
            list: env_contents
                .split(',')
                .map(String::from)
                .collect::<Vec<String>>()
                .into_boxed_slice(),
        }
    }

    fn log<F>(&self, t: &str, f: F)
    where
        F: Fn();
}
