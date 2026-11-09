Name:           cosmic-settings-daemon
Epoch:          1
Version: 1.11.1
Release:        1%{?dist}
Summary:        COSMIC Settings Daemon (Playtron fork)

License:        GPL-3.0-only
URL:            https://github.com/pop-os/cosmic-settings-daemon
Source0:        %{name}.tar.gz

%global debug_package %{nil}

# Runtime dependencies. Shared libraries are auto-detected from the ELF; these
# are the services/tools the daemon shells out to or requires at runtime.
Requires:       geoclue2
Requires:       wireplumber
Requires:       pulseaudio-utils
Requires:       polkit
Requires:       acpid
Requires:       adw-gtk3-theme

# Override the stock cosmic-settings-daemon from the distro. The Epoch makes our
# fork win regardless of the upstream version number.
Provides:       cosmic-settings-daemon = %{epoch}:%{version}-%{release}
Obsoletes:      cosmic-settings-daemon < %{epoch}:%{version}

%description
Settings daemon for the COSMIC desktop environment. Handles display and keyboard
brightness, theme auto-switching, audio, input, and automatic timezone detection
(geoclue + timezone boundary lookup).

%prep
%autosetup -n %{name} -p1

%build

%install
install -Dm0755 "usr/bin/cosmic-settings-daemon" "%{buildroot}%{_bindir}/cosmic-settings-daemon"
install -Dm0644 "usr/share/cosmic/com.system76.CosmicSettings.Shortcuts/v1/system_actions" "%{buildroot}%{_datadir}/cosmic/com.system76.CosmicSettings.Shortcuts/v1/system_actions"
install -Dm0644 "usr/share/polkit-1/rules.d/cosmic-settings-daemon.rules" "%{buildroot}%{_datadir}/polkit-1/rules.d/cosmic-settings-daemon.rules"
install -Dm0644 "usr/share/licenses/cosmic-settings-daemon/LICENSE" "%{buildroot}%{_datadir}/licenses/cosmic-settings-daemon/LICENSE"

%files
%license %{_datadir}/licenses/cosmic-settings-daemon/LICENSE
%{_bindir}/cosmic-settings-daemon
%{_datadir}/cosmic/com.system76.CosmicSettings.Shortcuts/v1/system_actions
%{_datadir}/polkit-1/rules.d/cosmic-settings-daemon.rules

%changelog
* Wed Jun 11 2026 Playtron <dev@playtron.one> - 0.1.0-1
- Initial Playtron RPM package
- Add automatic timezone detection (geoclue + timedated)
