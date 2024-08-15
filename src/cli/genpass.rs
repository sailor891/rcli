use crate::{process_genpass, CmdExcutor};
use clap::Parser;
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, value_parser=clap::value_parser!(u8).range(8..),default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(short, long, default_value_t = true)]
    pub numbers: bool,

    #[arg(long, default_value_t = true)]
    pub symbols: bool,
}
impl CmdExcutor for GenPassOpts {
    async fn excutor(self) -> anyhow::Result<()> {
        let password = process_genpass(
            self.uppercase,
            self.lowercase,
            self.numbers,
            self.symbols,
            self.length,
        )?;
        println!("{}", password);
        let estimate = zxcvbn(&password, &[]);
        eprintln!("Password strength:{}", estimate.score());
        Ok(())
    }
}
