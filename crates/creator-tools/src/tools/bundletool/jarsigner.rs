use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Jarsigner {
    /// Specifies the URL that tells the keystore location. This defaults to the file
    /// .keystore in the user's home directory, as determined by the user.home system
    /// property.
    ///
    /// A keystore is required when signing. You must explicitly specify a keystore when
    /// the default keystore does not exist or if you want to use one other than the
    /// default.
    ///
    /// A keystore is not required when verifying, but if one is specified or the default
    /// exists and the -verbose option was also specified, then additional information
    /// is output regarding whether or not any of the certificates used to verify the
    /// JAR file are contained in that keystore.
    ///
    /// The -keystore argument can be a file
    /// name and path specification rather than a URL, in which case it is treated the
    /// same as a file: URL, for example, the following are equivalent:
    ///
    /// `-keystore filePathAndName`
    ///
    /// `-keystore file:filePathAndName`
    ///
    /// If the Sun PKCS #11 provider was configured in the java.security security
    /// properties file (located in the JRE's $JAVA_HOME/lib/security directory), then the
    /// keytool and jarsigner tools can operate on the PKCS #11 token by specifying these
    /// options:
    ///
    /// `-keystore NONE`
    ///
    /// `-storetype PKCS11`
    ///
    /// For example, the following command lists the contents of the configured PKCS#11
    /// token:
    ///
    /// `keytool -keystore NONE -storetype PKCS11 -list`
    keystore: Option<PathBuf>, // to do
    /// Specifies the password that is required to access the keystore. This is only
    /// needed when signing (not verifying) a JAR file. In that case, if a -storepass
    /// option is not provided at the command line, then the user is prompted for the
    /// password. If the modifier env or file is not specified, then the password has
    /// the value argument. Otherwise, the password is retrieved as follows:
    ///
    /// * env: Retrieve the password from the environment variable named argument.
    /// * file: Retrieve the password from the file named argument.
    ///
    /// ## Note:
    /// The password should not be specified on the command line or in a script unless it
    /// is for testing purposes, or you are on a secure system.
    storepass: Option<String>,
    /// Specifies the type of keystore to be instantiated. The default keystore type is
    /// the one that is specified as the value of the keystore.type property in the
    /// security properties file, which is returned by the static getDefaultType method in
    /// java.security.KeyStore.    
    ///
    /// The PIN for a PCKS #11 token can also be specified with the -storepass option. If
    /// none is specified, then the keytool and jarsigner commands prompt for the
    /// token PIN. If the token has a protected authentication path (such as a
    /// dedicated PIN-pad or a biometric reader), then the -protected option must be
    /// specified and no password options can be specified.
    storetype: Option<String>, // to do
    /// Specifies the password used to protect the private key of the keystore entry
    /// addressed by the alias specified on the command line. The password is required
    /// when using jarsigner to sign a JAR file. If no password is provided on the command
    /// line, and the required password is different from the store password, then the
    /// user is prompted for it.
    ///
    /// If the modifier env or file is not specified, then the password has the value
    /// argument. Otherwise, the password is retrieved as follows:
    ///
    /// * env: Retrieve the password from the environment variable named argument.
    /// * file: Retrieve the password from the file named argument.
    ///
    /// ## Note:
    /// The password should not be specified on the command line or in a script unless it
    /// is for testing purposes, or you are on a secure system.
    keypass: Option<String>,
    /// Specifies the certificate chain to be used when the certificate chain associated
    /// with the private key of the keystore entry that is addressed by the alias
    /// specified  on the command line is not complete. This can happen when the keystore
    /// is located on a hardware token where there is not enough capacity to hold a
    /// complete certificate chain. The file can be a sequence of concatenated X.509
    /// certificates, or a single PKCS#7 formatted data block, either in binary
    /// encoding format or in printable encoding format (also known as Base64
    /// encoding) as defined by the Internet RFC 1421 standard. See the section
    /// Internet RFC 1421 Certificate Encoding Standard in keytool and http://tools.ietf.org/html/rfc1421.
    certchain: Option<PathBuf>,
    /// Specifies the base file name to be used for the generated .SF and .DSA files. For
    /// example, if file is DUKESIGN, then the generated .SF and .DSA files are named
    /// DUKESIGN.SF and DUKESIGN.DSA, and placed in the META-INF directory of the signed
    /// JAR file.
    ///
    /// The characters in the file must come from the set a-zA-Z0-9_-. Only
    /// letters, numbers, underscore, and hyphen characters are allowed. All lowercase
    /// characters are converted to uppercase for the .SF and .DSA file names.
    ///
    /// If no -sigfile option appears on the command line, then the base file name for the
    /// .SF and .DSA files is the first 8 characters of the alias name specified on the
    /// command line, all converted to upper case. If the alias name has fewer than 8
    /// characters, then the full alias name is used. If the alias name contains any
    /// characters that are not valid in a signature file name, then each such character
    /// is converted to an underscore (_) character to form the file name.
    sigfile: Option<PathBuf>,
    /// Name of signed JAR file
    signedjar: Option<PathBuf>,
    /// Specifies the name of the message digest algorithm to use when digesting the
    /// entries of a JAR file.
    ///
    /// For a list of standard message digest algorithm names, see "Appendix A: Standard
    /// Names" in the Java Cryptography Architecture (JCA) Reference Guide at [`here`]
    /// If this option is not specified, then SHA256 is used. There must either be a
    /// statically installed provider supplying an implementation of the specified
    /// algorithm or the user must specify one with the -providerClass option; otherwise,
    /// the command will not succeed.
    ///
    /// [`here`]::http://docs.oracle.com/javase/7/docs/technotes/guides/security/crypto/CryptoSpec.html#AppA
    digestalg: Option<String>,
    /// Specifies the name of the signature algorithm to use to sign the JAR file.
    ///
    /// For a list of standard signature algorithm names, see "Appendix A: Standard Names"
    /// in the Java Cryptography Architecture (JCA) Reference Guide at [`here`]
    ///
    /// This algorithm must be compatible with the private key used to sign the JAR file.
    /// If this option is not specified, then SHA1withDSA, SHA256withRSA, or
    /// SHA256withECDSA are used depending on the type of private key. There must either
    /// be a statically installed provider supplying an implementation of the specified
    /// algorithm or the user must specify one with the -providerClass option; otherwise,
    /// the command will not succeed.
    ///
    /// [`here`]::http://docs.oracle.com/javase/7/docs/technotes/guides/security/crypto/CryptoSpec.html#AppA
    sigalg: Option<String>, // to do
    /// Verify a signed JAR file
    verify: bool,
    ///When the -verbose option appears on the command line, it indicates verbose mode,
    /// which causes jarsigner to output extra information about the progress of the JAR
    /// signing or verification.
    verbose: Option<String>, // to do
    /// If the -certs option appears on the command line with the -verify and -verbose
    /// options, then the output includes certificate information for each signer of the
    /// JAR file. This information includes the name of the type of certificate (stored in
    /// the .DSA file) that certifies the signer's public key, and if the certificate is
    /// an X.509 certificate (an instance of the `java.security.cert.X509Certificate`),
    /// then the distinguished name of the signer.
    ///
    /// The keystore is also examined. If no keystore value is specified on the command
    /// line, then the default keystore file (if any) is checked. If the public key
    /// certificate for a signer matches an entry in the keystore, then the alias name for
    /// the keystore entry for that signer is displayed in parentheses. If the signer
    /// comes from a JDK 1.1 identity database instead of from a keystore, then the alias
    /// name displays in brackets instead of parentheses.
    certs: bool,
    ///
    tsa: Option<PathBuf>,
    /// In the past, the .DSA (signature block) file generated when a JAR file was signed
    /// included a complete encoded copy of the .SF file (signature file) also generated.
    /// This behavior has been changed. To reduce the overall size of the output JAR file,
    /// the .DSA file by default does not contain a copy of the .SF file anymore. If
    /// -internalsf appears on the command line, then the old behavior is utilized. This
    /// option is useful for testing. In practice, do not use the -internalsf option
    /// because it incurs higher overhead.
    internalsf: bool,
    /// If the -sectionsonly option appears on the command line, then the .SF file
    /// (signature file) generated when a JAR file is signed does not include a header
    /// that contains a hash of the whole manifest file. It contains only the information
    /// and hashes related to each individual source file included in the JAR file. See
    /// Signature File.
    ///
    /// By default, this header is added, as an optimization. When the header is present,
    /// whenever the JAR file is verified, the verification can first check to see whether
    /// the hash in the header matches the hash of the whole manifest file. When there is
    /// a match, verification proceeds to the next step. When there is no match, it is
    /// necessary to do a less optimized verification that the hash in each source file
    /// information section in the .SF file equals the hash of its corresponding section
    /// in the manifest file. See [`JAR File Verification`].
    ///
    /// The -sectionsonly option is primarily used for testing. It should not be used
    /// other than for testing because using it incurs higher overhead.
    ///
    /// [`JAR File Verification`]::https://docs.oracle.com/javase/7/docs/technotes/tools/windows/jarsigner.html#CCHDAJHB
    sectionsonly: bool,
    /// Values can be either true or false. Specify true when a password must be specified
    /// through a protected authentication path such as a dedicated PIN reader.
    protected: bool,
    /// Used to specify the name of cryptographic service provider's master class file
    /// when the service provider is not listed in the java.security security properties
    /// file.
    ///
    /// Used with the -providerArg ConfigFilePath option, the keytool and jarsigner tools
    /// install the provider dynamically and use ConfigFilePath for the path to the token
    /// configuration file. The following example shows a command to list a PKCS #11
    /// keystore when the Oracle PKCS #11 provider was not configured in the security
    /// properties file.
    ///
    /// ```
    /// jarsigner -keystore NONE -storetype PKCS11 \
    ///     -providerClass sun.security.pkcs11.SunPKCS11 \
    ///     -providerArg /mydir1/mydir2/token.config \
    ///     -list
    /// ```
    provider_class: Option<String>,
    /// If more than one provider was configured in the java.security security properties
    /// file, then you can use the -providerName option to target a specific provider
    /// instance. The argument to this option is the name of the provider.
    ///
    /// For the Oracle PKCS #11 provider, providerName is of the form SunPKCS11-TokenName,
    /// where TokenName is the name suffix that the provider instance has been configured
    /// with, as detailed in the configuration attributes table. For example, the
    /// following command lists the contents of the PKCS #11 keystore provider instance
    /// with name suffix SmartCard:
    ///
    /// ```
    /// jarsigner -keystore NONE -storetype PKCS11 \
    ///     -providerName SunPKCS11-SmartCard \
    ///     -list
    /// ```
    provider_name: Option<String>,
    /// When -tsacert alias appears on the command line when signing a JAR file, a time
    /// stamp is generated for the signature. The alias identifies the TSA public key
    /// certificate in the keystore that is in effect. The entry's certificate is examined
    /// for a Subject Information Access extension that contains a URL identifying the
    /// location of the TSA.
    ///
    /// The TSA public key certificate must be present in the keystore when using the
    /// `-tsacert` option.
    tsacert: Option<PathBuf>,
    /// Specifies the object identifier (OID) that identifies the policy ID to be sent to
    /// the TSA server. If this option is not specified, no policy ID is sent and the TSA
    /// server will choose a default policy ID.
    ///
    /// Object identifiers are defined by X.696, which is an ITU Telecommunication
    /// Standardization Sector (ITU-T) standard. These identifiers are typically
    /// period-separated sets of non-negative digits like 1.2.3.4, for example.
    tsapolicyid: Option<String>,
    /// algorithm of digest data in timestamping request
    tsadigestalg: Option<String>,
    /// This option specifies an alternative signing mechanism. The fully qualified class
    /// name identifies a class file that extends the com.sun.jarsigner.ContentSigner
    /// abstract class. The path to this class file is defined by the -altsignerpath
    /// option. If the -altsigner option is used, then the jarsigner command uses the
    /// signing mechanism provided by the specified class. Otherwise, the jarsigner
    /// command uses its default signing mechanism.
    ///
    /// For example, to use the signing mechanism provided by a class named
    /// `com.sun.sun.jarsigner.AuthSigner`, use the jarsigner option `-altsigner`
    /// `com.sun.jarsigner.AuthSigner`.
    altsigner: Option<PathBuf>,
    /// Specifies the path to the class file and any JAR file it depends on. The class
    /// file name is specified with the -altsigner option. If the class file is in a JAR
    /// file, then this option specifies the path to that JAR file.
    ///
    /// An absolute path or a path relative to the current directory can be specified. If
    /// classpathlist contains multiple paths or JAR files, then they should be separated
    /// with a colon (:) on Oracle Solaris and a semicolon (;) on Windows. This option is
    /// not necessary when the class is already in the search path.
    ///
    /// The following example shows how to specify the path to a JAR file that contains
    /// the class file. The JAR file name is included.
    ///
    /// ```
    /// -altsignerpath /home/user/lib/authsigner.jar
    /// ```
    ///
    /// The following example shows how to specify the path to the JAR file that contains
    /// the class file. The JAR file name is omitted.
    ///
    /// ```
    /// -altsignerpath /home/user/classes/com/sun/tools/jarsigner/
    /// ```
    altsignerpath: Vec<PathBuf>,
    /// During the signing or verifying process, the command may issue warning messages.
    /// If you specify this option, the exit code of the tool reflects the severe warning
    /// messages that this command found. See Errors and Warnings.
    strict: bool,
}

impl Jarsigner {
    pub fn new() -> Self {
        Self {
            keystore: None,
            storepass: None,
            storetype: None,
            keypass: None,
            certchain: None,
            sigfile: None,
            signedjar: None,
            digestalg: None,
            sigalg: None,
            verify: false,
            verbose: None,
            certs: false,
            tsa: None,
            internalsf: false,
            sectionsonly: false,
            protected: false,
            provider_class: None,
            provider_name: None,
            tsacert: None,
            tsapolicyid: None,
            tsadigestalg: None,
            altsigner: None,
            altsignerpath: Vec::new(),
            strict: false,
        }
    }

    pub fn keystore(&mut self, keystore: &Path) -> &mut Self {
        self.keystore = Some(keystore.to_owned());
        self
    }

    pub fn storepass(&mut self, storepass: String) -> &mut Self {
        self.storepass = Some(storepass);
        self
    }

    pub fn storetype(&mut self, storetype: String) -> &mut Self {
        self.storetype = Some(storetype);
        self
    }

    pub fn keypass(&mut self, keypass: String) -> &mut Self {
        self.keypass = Some(keypass);
        self
    }

    pub fn certchain(&mut self, certchain: &Path) -> &mut Self {
        self.certchain = Some(certchain.to_owned());
        self
    }

    pub fn sigfile(&mut self, sigfile: &Path) -> &mut Self {
        self.sigfile = Some(sigfile.to_owned());
        self
    }
    pub fn signedjar(&mut self, signedjar: &Path) -> &mut Self {
        self.signedjar = Some(signedjar.to_owned());
        self
    }

    pub fn digestalg(&mut self, digestalg: String) -> &mut Self {
        self.digestalg = Some(digestalg);
        self
    }

    pub fn sigalg(&mut self, sigalg: String) -> &mut Self {
        self.sigalg = Some(sigalg);
        self
    }

    pub fn verify(&mut self, verify: bool) -> &mut Self {
        self.verify = verify;
        self
    }

    pub fn verbose(&mut self, verbose: String) -> &mut Self {
        self.verbose = Some(verbose);
        self
    }

    pub fn certs(&mut self, certs: bool) -> &mut Self {
        self.certs = certs;
        self
    }

    pub fn tsa(&mut self, tsa: &Path) -> &mut Self {
        self.tsa = Some(tsa.to_owned());
        self
    }

    pub fn internalsf(&mut self, internalsf: bool) -> &mut Self {
        self.internalsf = internalsf;
        self
    }

    pub fn sectionsonly(&mut self, sectionsonly: bool) -> &mut Self {
        self.sectionsonly = sectionsonly;
        self
    }

    pub fn protected(&mut self, protected: bool) -> &mut Self {
        self.protected = protected;
        self
    }

    pub fn provider_class(&mut self, provider_class: String) -> &mut Self {
        self.provider_class = Some(provider_class);
        self
    }

    pub fn provider_name(&mut self, provider_name: String) -> &mut Self {
        self.provider_name = Some(provider_name);
        self
    }

    pub fn tsacert(&mut self, tsacert: &Path) -> &mut Self {
        self.tsacert = Some(tsacert.to_owned());
        self
    }

    pub fn tsapolicyid(&mut self, tsapolicyid: String) -> &mut Self {
        self.tsapolicyid = Some(tsapolicyid);
        self
    }

    pub fn tsadigestalg(&mut self, tsadigestalg: String) -> &mut Self {
        self.tsadigestalg = Some(tsadigestalg);
        self
    }

    pub fn altsigner(&mut self, altsigner: &Path) -> &mut Self {
        self.altsigner = Some(altsigner.to_owned());
        self
    }

    pub fn altsignerpath(&mut self, altsignerpath: &Path) -> &mut Self {
        self.altsignerpath.push(altsignerpath.to_path_buf());
        self
    }

    pub fn strict(&mut self, strict: bool) -> &mut Self {
        self.strict = strict;
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut jarsigner = Command::new("jarsigner");
        if let Some(keystore) = &self.keystore {
            jarsigner.arg("-keystore").arg(keystore);
        }
        if let Some(storepass) = &self.storepass {
            jarsigner.arg("-storepass").arg(storepass);
        }
        if let Some(storetype) = &self.storetype {
            jarsigner.arg("-storetype").arg(storetype);
        }
        if let Some(keypass) = &self.keypass {
            jarsigner.arg("-skeypass").arg(keypass);
        }
        if let Some(certchain) = &self.certchain {
            jarsigner.arg("-certchain").arg(certchain);
        }
        if let Some(sigfile) = &self.sigfile {
            jarsigner.arg("-sigfile").arg(sigfile);
        }
        if let Some(signedjar) = &self.signedjar {
            jarsigner.arg("-signedjar").arg(signedjar);
        }
        if let Some(digestalg) = &self.digestalg {
            jarsigner.arg("-digestalg").arg(digestalg);
        }
        if let Some(sigalg) = &self.sigalg {
            jarsigner.arg("-sigalg").arg(sigalg);
        }
        if self.verify {
            jarsigner.arg("-verify");
        }
        if let Some(verbose) = &self.verbose {
            jarsigner.arg("-verbose").arg(verbose);
        }
        if self.certs {
            jarsigner.arg("-certs");
        }
        if let Some(tsa) = &self.tsa {
            jarsigner.arg("-tsa").arg(tsa);
        }
        if self.internalsf {
            jarsigner.arg("-internalsf");
        }
        if self.sectionsonly {
            jarsigner.arg("-sectionsonly");
        }
        if self.protected {
            jarsigner.arg("-protected");
        }
        if let Some(provider_class) = &self.provider_class {
            jarsigner.arg("-providerClass").arg(provider_class);
        }
        if let Some(provider_name) = &self.provider_name {
            jarsigner.arg("-providerName").arg(provider_name);
        }
        if let Some(tsacert) = &self.tsacert {
            jarsigner.arg("-tsacert").arg(tsacert);
        }
        if let Some(tsapolicyid) = &self.tsapolicyid {
            jarsigner.arg("-tsapolicyid").arg(tsapolicyid);
        }
        if let Some(tsadigestalg) = &self.tsadigestalg {
            jarsigner.arg("-tsadigestalg").arg(tsadigestalg);
        }
        if let Some(altsigner) = &self.altsigner {
            jarsigner.arg("-altsigner").arg(altsigner);
        }
        jarsigner.arg(
            self.altsignerpath
                .iter()
                .map(|v| v.to_string_lossy().to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
        if self.strict {
            jarsigner.arg("-strict");
        }
        Ok(())
    }
}
