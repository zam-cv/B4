use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

pub fn get_ssl_acceptor() -> anyhow::Result<SslAcceptorBuilder> {
  let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
  builder.set_private_key_file("cert/key.pem", SslFiletype::PEM)?;
  builder.set_certificate_chain_file("cert/cert.pem")?;
  Ok(builder)
}