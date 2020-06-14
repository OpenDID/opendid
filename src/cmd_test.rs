use clap::{ ArgMatches, SubCommand, App, };
use secp256k1::{ Secp256k1, Message, SecretKey, PublicKey, };
use crate::util::XResult;
use crate::cmd::Command;

pub struct CommandTest {
}

impl Command for CommandTest {

    fn subcommand<'a>(&self) -> Option<App<'a, 'a>> {
        Some(SubCommand::with_name(self.name()).about("test"))
    }

    fn name(&self) -> &str {
        "test"
    }

    fn run(&self, _arg_matches: &ArgMatches, _sub_arg_matches: &ArgMatches) -> XResult<()> {
        println!("This is test command!");

        let secp = Secp256k1::new();
        let secret_key_bytes = hex::decode("bc60c7fcae1a43947cdff39f9a5b8812025809160cc7f69a73027bae7c105a71")?;
        let secret_key = SecretKey::from_slice(&secret_key_bytes[..])?;
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        println!("{}", secret_key);
        println!("{}", hex::encode(&public_key.serialize_uncompressed()[..]));

        let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");
        let k = crate::p256k1::P256k1SecretKey::from_hex("bc60c7fcae1a43947cdff39f9a5b8812025809160cc7f69a73027bae7c105a71")?;
        let sig = k.sign(&message);
        println!("SIG: {}", sig);

        let r = secp.verify(&message, &sig, &public_key).is_ok();
        println!("RESULT: {}", r);

        Ok(())
    }
}