use clap::Parser;
use crossbundle_tools::{error::CommandExt, tools::AndroidSdk, utils::Config, EXECUTABLE_SUFFIX_BAT};

#[derive(Parser, Clone, Debug, Default)]
pub struct SdkManagerInstallCommand {
    /// Install all preferred tools for correct crossbundle work
    #[clap(long, short)]
    preferred_tools: bool,
    /// List installed and available packages. Use the channel option to include a package from a channel up to and including channel_id.
    /// For example, specify the canary channel to list packages from all channels
    #[clap(long, short)]
    list: bool,
    /// Install package. To see all available packages use --list.
    /// Example: crossbundle install sdk-manager "ndk;23.1.7779620"
    #[clap(long, short)]
    install: Option<String>,
    /// Android package that needs to be uninstalled
    #[clap(long)]
    uninstall: Option<String>,
    /// Update all installed packages
    #[clap(long)]
    update: bool,
    /// Use the specified SDK path instead of the SDK containing this tool
    #[clap(long, short)]
    sdk_root: Option<std::path::PathBuf>,
    /// Include packages in channels up to and including channel_id. Available channels are:
    /// 0 (Stable), 1 (Beta), 2 (Dev), and 3 (Canary)
    #[clap(long, short)]
    channel: Option<u32>,
    /// Include obsolete packages in the package listing or package updates. For use with --list and --update only
    #[clap(long)]
    include_obsolete: bool,
    /// Force all connections to use HTTP rather than HTTPS
    #[clap(long, short)]
    no_https: bool,
    /// Verbose output mode. Errors, warnings and informational messages are printed
    #[clap(long, short)]
    verbose: bool,
    /// Connect via a proxy of the given type: either http for high level protocols such as HTTP or FTP,
    /// or socks for a SOCKS (V4 or V5) proxy
    #[clap(long)]
    proxy: Option<String>,
    /// IP or DNS address of the proxy to use
    #[clap(long)]
    proxy_host: Option<String>,
    /// Proxy port number to connect to
    #[clap(long)]
    proxy_port: Option<String>,
}

impl SdkManagerInstallCommand {
    /// Creates a new empty instance.
    pub fn new(&self) -> Self {
        Self {
            ..Default::default()
        }
    }

    /// List installed and available packages. Use the channel option to include a package from a channel up to and including channel_id.
    /// For example, specify the canary channel to list packages from all channels
    pub fn list(&mut self, list: bool) -> &mut Self {
        self.list = list;
        self
    }

    /// Install package. To see all available packages use --list.
    /// Example: crossbundle install sdk-manager "ndk;23.1.7779620"
    pub fn install(&mut self, install: String) -> &mut Self {
        self.install = Some(install);
        self
    }

    /// Android package that needs to be uninstalled
    pub fn uninstall(&mut self, uninstall: String) -> &mut Self {
        self.uninstall = Some(uninstall);
        self
    }

    /// Update all installed packages
    pub fn update(&mut self, update: bool) -> &mut Self {
        self.update = update;
        self
    }

    /// Install all required tools for correct crossbundle work
    pub fn preferred_tools(&mut self, preferred_tools: bool) -> &mut Self {
        self.preferred_tools = preferred_tools;
        self
    }

    /// Use the specified SDK path instead of the SDK containing this tool
    ///  ```sh
    /// --sdk_root=path
    /// ```
    pub fn sdk_root(&mut self, sdk_root: std::path::PathBuf) -> &mut Self {
        self.sdk_root = Some(sdk_root);
        self
    }

    /// Include packages in channels up to and including channel_id. Available channels are:
    /// 0 (Stable), 1 (Beta), 2 (Dev), and 3 (Canary).
    /// ```sh
    /// --channel=channel_id
    /// ```
    pub fn channel(&mut self, channel: u32) -> &mut Self {
        self.channel = Some(channel);
        self
    }

    /// Include obsolete packages in the package listing or package updates. For use with --list and --update only.
    pub fn include_obsolete(&mut self, include_obsolete: bool) -> &mut Self {
        self.include_obsolete = include_obsolete;
        self
    }

    /// Force all connections to use HTTP rather than HTTPS.
    pub fn no_https(&mut self, no_https: bool) -> &mut Self {
        self.no_https = no_https;
        self
    }

    /// Verbose output mode. Errors, warnings and informational messages are printed.
    pub fn verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose = verbose;
        self
    }

    /// Connect via a proxy of the given type: either http for high level protocols such as HTTP or FTP, or socks for a SOCKS (V4 or V5) proxy.
    /// ```sh
    /// --proxy={http | socks}
    /// ```
    pub fn proxy(&mut self, proxy: String) -> &mut Self {
        self.proxy = Some(proxy);
        self
    }

    /// IP or DNS address of the proxy to use.
    /// ```sh
    /// --proxy_host={IP_address | DNS_address}
    /// ```
    pub fn proxy_host(&mut self, proxy_host: String) -> &mut Self {
        self.proxy_host = Some(proxy_host);
        self
    }

    /// Proxy port number to connect to.
    /// ```sh
    /// --proxy_port=port_number
    /// ```
    pub fn proxy_port(&mut self, proxy_port: String) -> &mut Self {
        self.proxy_port = Some(proxy_port);
        self
    }

    /// Run sdkmanager command with specified flags and options
    pub fn run(&self, _config: &Config) -> crate::error::Result<()> {
        let sdk_root = AndroidSdk::sdk_install_path()?;
        let sdkmanager_path = sdk_root.join("cmdline-tools").join("bin");
        let sdkmanager_bat =
            sdkmanager_path.join(format!("sdkmanager{}", EXECUTABLE_SUFFIX_BAT));

        let mut sdkmanager = std::process::Command::new(sdkmanager_bat);
        if let Some(sdk_root) = &self.sdk_root {
            sdkmanager.arg(sdk_root);
        } else {
            sdkmanager.arg(format!("--sdk_root={}", sdk_root.to_str().unwrap()));
        }
        // TODO: Resolve the problem about installation several packages
        if let Some(install) = &self.install {
            sdkmanager.arg(install);
        }
        if let Some(uninstall) = &self.uninstall {
            sdkmanager.arg("--uninstall").arg(uninstall);
        }
        if self.update {
            sdkmanager.arg("--update");
        }
        if self.list {
            sdkmanager.arg("--list");
        }
        if self.preferred_tools {
            sdkmanager
                .arg("build-tools;29.0.0")
                .arg("ndk;23.1.7779620")
                .arg("platforms;android-30");
        }
        if let Some(channel) = &self.channel {
            sdkmanager.arg(format!("--channel={}", channel));
        }
        if self.include_obsolete {
            sdkmanager.arg("--include_obsolete");
        }
        if self.no_https {
            sdkmanager.arg("--no_https");
        }
        if self.verbose {
            sdkmanager.arg("--verbose");
        }
        if let Some(http_or_socks) = &self.proxy {
            sdkmanager.arg(format!("--proxy={}", http_or_socks));
        }
        if let Some(ip_or_dns) = &self.proxy_host {
            sdkmanager.arg(format!("--proxy_host={}", ip_or_dns));
        }
        if let Some(port_number) = &self.proxy_port {
            sdkmanager.arg(format!("--proxy_port={}", port_number));
        }
        sdkmanager.output_err(true)?;
        Ok(())
    }
}
