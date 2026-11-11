## [1.11.2](https://github.com/kora-desktop/cosmic-settings-daemon/compare/v1.11.1...v1.11.2) (2026-11-11)


### Bug Fixes

* **audio:** announce a default sink rebuilt under the same name ([b2c5ced](https://github.com/kora-desktop/cosmic-settings-daemon/commit/b2c5cedb167e1152e1b862ba43043fd42f1b9983))
* **shortcuts:** launch kora-launcher, kora-panel and kora-settings ([22848d3](https://github.com/kora-desktop/cosmic-settings-daemon/commit/22848d3a90b33a75e37f8fb0011f74ceb58c55c5))

## [1.11.1](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.11.0...v1.11.1) (2026-11-09)


### Bug Fixes

* **shortcuts:** open Chord instead of the removed chat panel ([6f25d1a](https://github.com/playtron-os/cosmic-settings-daemon/commit/6f25d1a5fef68bc84487157d1cf9f18f1cb18556))

# [1.11.0](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.10.0...v1.11.0) (2026-10-23)


### Features

* **shortcuts:** read distribution overrides of system actions ([48c439e](https://github.com/playtron-os/cosmic-settings-daemon/commit/48c439e30d8a305b98d7baff467196b5af40a318))

# [1.10.0](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.9.0...v1.10.0) (2026-10-21)


### Bug Fixes

* **theme:** set GTK keys with dconf when gsettings has no schema ([2c774e9](https://github.com/playtron-os/cosmic-settings-daemon/commit/2c774e9f72425dda2c00c90fc5bcc101e836b58d))


### Features

* **shortcuts:** add the screen capture actions ([6ae5c19](https://github.com/playtron-os/cosmic-settings-daemon/commit/6ae5c195d4b672bed594e93379721c21df0fed5e))

# [1.9.0](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.8.0...v1.9.0) (2026-10-20)


### Features

* **shortcuts:** add the SpeechToText action ([e683b5b](https://github.com/playtron-os/cosmic-settings-daemon/commit/e683b5b7b8e155f0d3293cf3d555df9b883d8a48))

# [1.8.0](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.7.0...v1.8.0) (2026-10-11)


### Features

* **theme:** give GTK apps the active icetron theme ([49b6ed9](https://github.com/playtron-os/cosmic-settings-daemon/commit/49b6ed90578090c0bfd376baeb5fb5deb19d1205))

# [1.7.0](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.6.0...v1.7.0) (2026-10-06)


### Features

* **shortcuts:** add the FocusHalo and MoveToDesktop actions ([43b0c06](https://github.com/playtron-os/cosmic-settings-daemon/commit/43b0c069764ead33fc9a8c49c65b08b7fe8ffe28))
* **shortcuts:** add the WindowCommands action ([841b714](https://github.com/playtron-os/cosmic-settings-daemon/commit/841b7148c9e2f7a29d0b322223a47f254a60cf13))

# [1.6.0](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.5.0...v1.6.0) (2026-09-11)


### Bug Fixes

* drop dist:rpm:kora, and match the profile element by regex ([73a55a7](https://github.com/playtron-os/cosmic-settings-daemon/commit/73a55a710ce7e65f52ce943f658edb1252f9d4e8))
* resolve sound and GTK themes through XDG data dirs, not /usr/share ([a57d2e7](https://github.com/playtron-os/cosmic-settings-daemon/commit/a57d2e7b89c13062c0941c7d0e0736805f252f5d))


### Features

* add deploy:nix for remote Kora hosts ([ea4cd11](https://github.com/playtron-os/cosmic-settings-daemon/commit/ea4cd1104eee3095da680d3c1419c3680131e347))
* add NextRealm and PreviousRealm shortcut actions ([acd37cc](https://github.com/playtron-os/cosmic-settings-daemon/commit/acd37ccd2d45765a2f9de7432aa706f40e437046))

# [1.5.0](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.4.1...v1.5.0) (2026-08-14)


### Features

* add calculator action ([790396f](https://github.com/playtron-os/cosmic-settings-daemon/commit/790396fbd6fcb5c0369e623d578d6a826b3ecbe9))
* forward this session's settings to a shared compositor ([fa2260a](https://github.com/playtron-os/cosmic-settings-daemon/commit/fa2260a4db20cc2c33795858a3e4ab2f41e9a1ec))

## [1.4.1](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.4.0...v1.4.1) (2026-08-04)


### Bug Fixes

* add retry for wayland connection ([5cc6c5e](https://github.com/playtron-os/cosmic-settings-daemon/commit/5cc6c5eb5564e1f1f7526593a43119f88749760a))

# [1.4.0](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.3.1...v1.4.0) (2026-07-29)


### Bug Fixes

* **audio-backend:** adjust profile priority 1/2 ([97f3e75](https://github.com/playtron-os/cosmic-settings-daemon/commit/97f3e758010f17d8bf02910399c1bb1aef9660c7))
* **audio-backend:** adjust profile priority 2/2 ([65d4616](https://github.com/playtron-os/cosmic-settings-daemon/commit/65d4616279991cab02be8e4a24bfcbd133912527))
* **audio-backend:** avoid node volume overwrite if set before node was available ([e60ea40](https://github.com/playtron-os/cosmic-settings-daemon/commit/e60ea405fd6a06c682d0822e2000a752afa4cfad))
* **audio-backend:** on default node set, check before assuming a node is a source ([3b5ef44](https://github.com/playtron-os/cosmic-settings-daemon/commit/3b5ef4432b44114b1d940d05f0317f1243e7d561))
* **audio-server:** active profile messages not being emitted ([9d5167a](https://github.com/playtron-os/cosmic-settings-daemon/commit/9d5167abceb3dd518d3632411b0301565626b988))
* **audio-server:** also consider source routes when detecting headset profiles ([ba9dfdf](https://github.com/playtron-os/cosmic-settings-daemon/commit/ba9dfdf839b8c98c6105941d30ec710217bf4f90))
* **audio-server:** always emit ActiveRoute signals to fix device names ([bf5b5e4](https://github.com/playtron-os/cosmic-settings-daemon/commit/bf5b5e40c44abfeb61720b39ca8e75520b62e220))
* **audio-server:** detect when a speaker route changes to a headphone ([0319a6b](https://github.com/playtron-os/cosmic-settings-daemon/commit/0319a6b16ea680b5805faf304253393c903f811a))
* **audio-server:** disable headset profile detection for bluetooth devices ([51cf44b](https://github.com/playtron-os/cosmic-settings-daemon/commit/51cf44ba2c75b0c56841ba11c2123e3d48543b82))
* **audio-server:** do not skip headset detection for devices available on startup ([e782308](https://github.com/playtron-os/cosmic-settings-daemon/commit/e782308cf201a7ea90c34d249d6ca2ae01c19019))
* **audio-server:** ensure headset profile route is a source ([c46a159](https://github.com/playtron-os/cosmic-settings-daemon/commit/c46a15903472482f5871645c1e52cbf39f532e7f))
* **audio-server:** handle default source/sink event even if name is unchanged ([cc67a7a](https://github.com/playtron-os/cosmic-settings-daemon/commit/cc67a7afc2d36b62715fb93908585c347fbc4f7f))
* **audio-server:** missing argument to cosmic-osd ([5e84219](https://github.com/playtron-os/cosmic-settings-daemon/commit/5e84219b9ae331302fc3c452f19da9f532773469))
* **audio-server:** on headset profile set, set route if profile is already active ([e9c15eb](https://github.com/playtron-os/cosmic-settings-daemon/commit/e9c15eb47c4cc39fa9734c8fd1c023b83258cf02))
* **audio-server:** on setting headphone profile, attempt to active available internal mic route ([09ae5e0](https://github.com/playtron-os/cosmic-settings-daemon/commit/09ae5e0b366b4bebaa07387b868dfecce16450c6))
* **audio-server:** only show dialog for physical plugs (no bt) ([169d325](https://github.com/playtron-os/cosmic-settings-daemon/commit/169d325c3c42935b4664f8ecea68008b1bfd4a59))
* **audio-server:** reset headset check when headset route is unplugged ([4a55b8e](https://github.com/playtron-os/cosmic-settings-daemon/commit/4a55b8e15325c466808d9357190623cd2c822a10))
* **audio-server:** select headset route which has headset port type ([d4a009b](https://github.com/playtron-os/cosmic-settings-daemon/commit/d4a009b08ed4db99effe3c6a2c758eae9a22c1cd))
* **audio-server:** sync source/sink volume/balance on default node change ([e7936f0](https://github.com/playtron-os/cosmic-settings-daemon/commit/e7936f0d4f01c8c5a436908c7ee9f6ad43e6a7d8))
* **audio-server:** use port type as fallback for headset detection ([3ccf428](https://github.com/playtron-os/cosmic-settings-daemon/commit/3ccf428129edd50d99f3e82154dd69c22736da1c))
* **audio-server:** volume set to 0 when a volume property update is empty ([612aa96](https://github.com/playtron-os/cosmic-settings-daemon/commit/612aa9600ad8f8f5a08b9de0f113768284e112ee))
* **audio:** round volume values to fix volume raise shortcut ([610589e](https://github.com/playtron-os/cosmic-settings-daemon/commit/610589e35ac111c68ea6c05c5f2dd952490a3045))
* **brightness:** drop cached DDC handles when idle ([21a9692](https://github.com/playtron-os/cosmic-settings-daemon/commit/21a9692b53fcbffa0f18f7d0a12bf0f9d5bd0590)), closes [#165](https://github.com/playtron-os/cosmic-settings-daemon/issues/165)
* CI dockerfile missing packages ([1ff380b](https://github.com/playtron-os/cosmic-settings-daemon/commit/1ff380bcff96491081b382e75a9c70c556ef31bf))
* clear routes and profiles on 0 index ([60382a4](https://github.com/playtron-os/cosmic-settings-daemon/commit/60382a4789f16cd3262d2cd4922396ab83c6dd74))
* **cosmic-pipewire:** check route::device when getting active route by card profile device ([d06c8b7](https://github.com/playtron-os/cosmic-settings-daemon/commit/d06c8b774594c092a0180ee6d180ee968f4f91a5))
* **cosmic-pipewire:** emit default source/sink event even if name is the same ([8ef1756](https://github.com/playtron-os/cosmic-settings-daemon/commit/8ef1756ffdaf21b51d649906e43dae4dd36d5db2))
* **cosmic-pipewire:** enumerate profiles after setting a profile ([6c1e69e](https://github.com/playtron-os/cosmic-settings-daemon/commit/6c1e69ed772ab763d56b2bc51b5ce5ea1fc776ac))
* **cosmic-pipewire:** RemoveDevice message not emitted ([1825703](https://github.com/playtron-os/cosmic-settings-daemon/commit/182570360d97d66273123c3623a0d0d428e3c992))
* **cosmic-pipewire:** request to enumerate routes after setting route ([9e6c38c](https://github.com/playtron-os/cosmic-settings-daemon/commit/9e6c38cfc3547772ef226917c3bccf64bd2746c3))
* **cosmic-pipewire:** retry pipewire connection on connection failure ([276fef9](https://github.com/playtron-os/cosmic-settings-daemon/commit/276fef9834b05c57b67e8ba36b74bea1ba690cee))
* **cosmic-pipewire:** route capacity panic ([5628d59](https://github.com/playtron-os/cosmic-settings-daemon/commit/5628d5932cbf1882a1b80e4e9123a0368c52eab3))
* dockerfile ([90873b7](https://github.com/playtron-os/cosmic-settings-daemon/commit/90873b796fc9d7dc2446c1e7c350c94fee2096bd))
* merge conflicts ([24122b3](https://github.com/playtron-os/cosmic-settings-daemon/commit/24122b3b261cb31419d5dde63d656375b880fd98))
* speaker route after headphone profile apply ([7d58eb9](https://github.com/playtron-os/cosmic-settings-daemon/commit/7d58eb94269e116cb77c9753f7f69a8c6b8fd90f))
* **theme:** unset flatpak override for QT_QPA_PLATFORMTHEME ([b5e6d82](https://github.com/playtron-os/cosmic-settings-daemon/commit/b5e6d827d5d7f4886864564c450d8ca096fa5167))
* update iced ([8c54bbb](https://github.com/playtron-os/cosmic-settings-daemon/commit/8c54bbbc10485c31e8717c21e119e19b87eeb3ea))


### Features

* **audio-server:** add SetRoute varlink method ([5c223bb](https://github.com/playtron-os/cosmic-settings-daemon/commit/5c223bbc4c26195fa7092ccf2a5f93d15a79a2cb))
* **audio-server:** pass better arguments to `cosmic-osd confirm-headphones` ([8f96be5](https://github.com/playtron-os/cosmic-settings-daemon/commit/8f96be58583430bbe9f18f56459f08a60ba3c41d))
* **audio-server:** volume step config ([b039821](https://github.com/playtron-os/cosmic-settings-daemon/commit/b03982175b0a72ebb5285db7094b3bc52ce7a9a8))
* graceful restart with SIGHUP ([75c0480](https://github.com/playtron-os/cosmic-settings-daemon/commit/75c0480f1315ec96ec3eb216fc939f93f57d7612))
* impl varlink service w/ audio APIs ([66d880b](https://github.com/playtron-os/cosmic-settings-daemon/commit/66d880bcc940af202ed64c0fa45c040345380c35))
* update MSRV to Rust 1.93 ([91f41b5](https://github.com/playtron-os/cosmic-settings-daemon/commit/91f41b59ab1cac13c240d9a22cefeb1879c75330))
* update pipewire/libspa to 0.10.0 ([24ab5c2](https://github.com/playtron-os/cosmic-settings-daemon/commit/24ab5c28eb5876665752cd5e9405b7de17e2b99a))
* update zlink to 0.5.0 ([ae95aca](https://github.com/playtron-os/cosmic-settings-daemon/commit/ae95acaef0664bc3ae11b39d7f6b01a3fd5df0ee))


### Performance Improvements

* **audio:** set device profiles internally with libpipewire ([9681bff](https://github.com/playtron-os/cosmic-settings-daemon/commit/9681bffe17ba16ad75359868811e9441f616cf7f))


### Reverts

* Revert "fix(audio-server): detect when a speaker route changes to a headphone" ([2a236f0](https://github.com/playtron-os/cosmic-settings-daemon/commit/2a236f0c13835bf4177b0e3aa921a0c0c6df3035))
* Revert "fix(audio-server): only show dialog for physical plugs (no bt)" ([64007d1](https://github.com/playtron-os/cosmic-settings-daemon/commit/64007d1164e1d2a554d13c228f0d398260236a43))

## [1.3.1](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.3.0...v1.3.1) (2026-07-16)


### Bug Fixes

* write is_dark values when they dont exist ([47f4af9](https://github.com/playtron-os/cosmic-settings-daemon/commit/47f4af9873deacdf992dba27b024e6b22b5d9931))

# [1.3.0](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.2.0...v1.3.0) (2026-06-27)


### Features

* add StartMenu system action ([c900ac4](https://github.com/playtron-os/cosmic-settings-daemon/commit/c900ac4ceec435edacae6d4f25eb56b7c530ebac))

# [1.2.0](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.1.0...v1.2.0) (2026-06-27)


### Features

* make window switcher action use agentos-launcher by default ([0f5d9a2](https://github.com/playtron-os/cosmic-settings-daemon/commit/0f5d9a2b46254fb6c90f4e4c39af641d2de8e9de))

# [1.1.0](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.0.1...v1.1.0) (2026-06-26)


### Features

* add spotlight and chatpanel actions ([72e5f44](https://github.com/playtron-os/cosmic-settings-daemon/commit/72e5f440625c9ad0353a2137b8e7dc6c37d3eff5))
* update system_actions to reflect correct actions ([f945e2a](https://github.com/playtron-os/cosmic-settings-daemon/commit/f945e2aa821cb2e9cbc574e7dbd78ecbe28169f5))

## [1.0.1](https://github.com/playtron-os/cosmic-settings-daemon/compare/v1.0.0...v1.0.1) (2026-06-12)


### Bug Fixes

* prevent post release failing ([d259185](https://github.com/playtron-os/cosmic-settings-daemon/commit/d259185b235f7f05b9b22ebe2f580a2e06b88865))

# 1.0.0 (2026-06-12)


### Bug Fixes

* allow modifier-less shortcut bindings for non-typing keys ([716da6d](https://github.com/playtron-os/cosmic-settings-daemon/commit/716da6d6af0b252e2f78aba2ad72ee19ae0241e0))
* also accept "wheel" as group name for polkit authentication ([99a6adc](https://github.com/playtron-os/cosmic-settings-daemon/commit/99a6adccd97e44b3968db9d194028b27cdcff0cd))
* **autoswitch:** delay until next event if theme is switched while autoswitch is enabled ([f4f4fb3](https://github.com/playtron-os/cosmic-settings-daemon/commit/f4f4fb39fb000bf8458404fe1629fb07cfd07235))
* avoid removing custom gtk css config files ([9d9ad8e](https://github.com/playtron-os/cosmic-settings-daemon/commit/9d9ad8ee7fef1b60ae030c4c9089addf547d2d0a))
* better calculation of next day's sunrise / sunset and handling of extremely inaccurate location updates ([bbc8965](https://github.com/playtron-os/cosmic-settings-daemon/commit/bbc896586658846a4e1ec6fc005b66923fa5ff5e))
* brightness not reaching 100 ([37806db](https://github.com/playtron-os/cosmic-settings-daemon/commit/37806db7411038cc9d42d56428fb69f4bc042f0e))
* clamp keyboard-driven backlight to 1% to prevent black screen ([d67de20](https://github.com/playtron-os/cosmic-settings-daemon/commit/d67de203ced04c5b8ec3486d93e0e61876a8ee91))
* **config:** Handle unknown Action/SystemAction gracefully ([b29dfc2](https://github.com/playtron-os/cosmic-settings-daemon/commit/b29dfc2ed0d913ccaaa3259b4ee8d5a90b025895))
* **cosmic-theme:** do not reset VS code config when unsetting theme configs ([#133](https://github.com/playtron-os/cosmic-settings-daemon/issues/133)) ([e37160f](https://github.com/playtron-os/cosmic-settings-daemon/commit/e37160f14d1e7ee428f973cd2848b4e95f83dfe1))
* deps ([19f1052](https://github.com/playtron-os/cosmic-settings-daemon/commit/19f10525ff00d76558147ea060bd856a87122353))
* different path for Config and State ([95378d8](https://github.com/playtron-os/cosmic-settings-daemon/commit/95378d8fbe3daf74f72469d585297e613be50671))
* do not exit on Geoclue failure ([a41d199](https://github.com/playtron-os/cosmic-settings-daemon/commit/a41d1991fdfdac58f0b788e43087ad6ee01d00ca))
* don't uppercase keysym first letter ([6833138](https://github.com/playtron-os/cosmic-settings-daemon/commit/68331387e4336c37ce2300bea8638257feab449a))
* ensure pop-sound-theme is installed ([add1cb3](https://github.com/playtron-os/cosmic-settings-daemon/commit/add1cb3c4a6c3557c78085d51eff9b1b80035020))
* gnome button layout differs from COSMIC on first login ([93c5494](https://github.com/playtron-os/cosmic-settings-daemon/commit/93c5494b3d01ff4fe745b12ccc4826b00f2e6897))
* handling for super binding ([eac9b17](https://github.com/playtron-os/cosmic-settings-daemon/commit/eac9b17070947d627d2450ee58a77d48377c511e))
* ignore location updates that are None ([39c92f7](https://github.com/playtron-os/cosmic-settings-daemon/commit/39c92f7b609201fc8417fcd70a1914fd4519ae92))
* install libpulse in dockerfile ([c6265d8](https://github.com/playtron-os/cosmic-settings-daemon/commit/c6265d86d022e8196a5f5e0dc6bd2bc66954bab4))
* Invert volume actions to be correct ([bb9eb90](https://github.com/playtron-os/cosmic-settings-daemon/commit/bb9eb904e800d09965e33883e4e15dfd011b1d19))
* keep timezone watcher alive ([d9d0891](https://github.com/playtron-os/cosmic-settings-daemon/commit/d9d089112c4671786e4530ba588ce0f10e99eb85))
* keyboard config reset on startup ([ae00cf5](https://github.com/playtron-os/cosmic-settings-daemon/commit/ae00cf5e1e0cd021089e40a8c655d30c3490a0be))
* missing icons in GTK apps ([181e8f9](https://github.com/playtron-os/cosmic-settings-daemon/commit/181e8f9c6269253f173f1bbcdd1385f23c78c598))
* only take oneshot for theme watcher if ID matches ([61c76a9](https://github.com/playtron-os/cosmic-settings-daemon/commit/61c76a9d060827402eeb9fe92cae73ce159d66e5))
* override until next auto-switch ([ea9d5f0](https://github.com/playtron-os/cosmic-settings-daemon/commit/ea9d5f030318bb66bd08f7adb630efc7f22596fb))
* **pulse:** pass headset port name to osd ([ff15f32](https://github.com/playtron-os/cosmic-settings-daemon/commit/ff15f3240f6cf36ea74eacbf55ad805377e88a41))
* **pulse:** reset tracked card on change with no unknown state ([6d45dbe](https://github.com/playtron-os/cosmic-settings-daemon/commit/6d45dbeaade7689ad2241f818fb1c6336ebe2bc2))
* remove debug log ([2f17f33](https://github.com/playtron-os/cosmic-settings-daemon/commit/2f17f33875315a4cf463f82c5dceca4d83a75bfd))
* reset critical battery alert if battery is normal ([8616c40](https://github.com/playtron-os/cosmic-settings-daemon/commit/8616c40d235164779cd3f2ceec1fe9b2b4aceb40))
* reset override if selected theme matches current time ([dd195a8](https://github.com/playtron-os/cosmic-settings-daemon/commit/dd195a8a2326462b20ce18bfb234df8d68bb2461))
* **shortcuts:** accept misc function keys as bindings that are set ([0613952](https://github.com/playtron-os/cosmic-settings-daemon/commit/0613952340e9327eb54cc9e20d7c649736db07b9))
* **shortcuts:** allow parsing bindings by name ([8d12bd1](https://github.com/playtron-os/cosmic-settings-daemon/commit/8d12bd17a34cd7fccc84bc402959a4fc69836124))
* **shortcuts:** Allow XF86 keysym range for shortcut bindings ([ebb2bd6](https://github.com/playtron-os/cosmic-settings-daemon/commit/ebb2bd61e309bf8363a78284647b9da363139ed6))
* **shortcuts:** do not uppercase key names to fix space ([b88b2af](https://github.com/playtron-os/cosmic-settings-daemon/commit/b88b2aff1c68ec94cddcae6877167c461c40b6e3))
* **shortcuts:** unmute on volume raise/lower ([eb886de](https://github.com/playtron-os/cosmic-settings-daemon/commit/eb886de5527f9b5d2f225708d63d9d36fbb63a64))
* switch light and dark despite suspend/time change ([6c3add2](https://github.com/playtron-os/cosmic-settings-daemon/commit/6c3add2e19e22296d53fb0b034bc05e8b64e7631))
* **system_actions:** Switch MuteMic to PipeWire ([6a633c5](https://github.com/playtron-os/cosmic-settings-daemon/commit/6a633c5bb7da1371e2b285b59fe9d0efb38afaf6))
* **system_actions:** use wpctl instead of amixer ([c5e8f72](https://github.com/playtron-os/cosmic-settings-daemon/commit/c5e8f7210c19ae0a4d1182db1ca0576aafc92c33))
* **theme:** don't unset the override if theme_mode darkness is unchanged ([4e9a2ae](https://github.com/playtron-os/cosmic-settings-daemon/commit/4e9a2ae3418f9779bc8d9292bdb923541d1a39b7))
* timezone-based theme auto-switching ([a36683f](https://github.com/playtron-os/cosmic-settings-daemon/commit/a36683fd5b7dea4011da278c91d0645117dc39f8))
* typo ([163d67e](https://github.com/playtron-os/cosmic-settings-daemon/commit/163d67e5b695057fef8aa77382c059ebf9fe374a))
* update libcosmic ([33b1847](https://github.com/playtron-os/cosmic-settings-daemon/commit/33b18479c0f26631524d0d600ed9bc413b6ca777))
* vscode theme parsing error ([54700df](https://github.com/playtron-os/cosmic-settings-daemon/commit/54700dfee57d1569efb2179896e36d754c2bf270))
* wait to manage theme until it has been watched ([3b2e1a1](https://github.com/playtron-os/cosmic-settings-daemon/commit/3b2e1a1fec657bb02a75d9bd95560b2dbc45226d))


### Features

* Add locale1 synchronization support ([0bc55bf](https://github.com/playtron-os/cosmic-settings-daemon/commit/0bc55bf00bf85ff93d6b9fff0fdb917c35b5b8d9))
* add ping / pong for removing configs that are no longer watched ([d89e456](https://github.com/playtron-os/cosmic-settings-daemon/commit/d89e45626537cbbb2768b0f83682bab5da7449eb))
* add support for offline and online timezone automatic detection, and RPM CI build ([49a16f2](https://github.com/playtron-os/cosmic-settings-daemon/commit/49a16f2b436f649233779ea3d3902cbb1e4fcf90))
* add system action and dbus method for input source switching ([e2aa105](https://github.com/playtron-os/cosmic-settings-daemon/commit/e2aa1056900d6f8c9c7555c0401aa7c99281eb06))
* apply changes to css files when theme variables change ([c5c0b69](https://github.com/playtron-os/cosmic-settings-daemon/commit/c5c0b6983b4282ac6e332b395c8373536a8a07fb))
* change gnome icon theme when cosmic icon theme changes ([a949447](https://github.com/playtron-os/cosmic-settings-daemon/commit/a949447cd4021ad1fc989c163e0952e92d3c08a8))
* cleanup gtk.css when exiting ([f079ab7](https://github.com/playtron-os/cosmic-settings-daemon/commit/f079ab7f98132787de385934915547f333dfbddc))
* **config:** document APIs and add methods for cosmic-settings ([5d01e28](https://github.com/playtron-os/cosmic-settings-daemon/commit/5d01e287112dca46cf96d58fa64c6fb0485b11a2))
* configurable keyboard shortcuts ([d643d97](https://github.com/playtron-os/cosmic-settings-daemon/commit/d643d97699bb9769676c4d60c486068751d83dd3))
* configurable volume actions ([ee782f4](https://github.com/playtron-os/cosmic-settings-daemon/commit/ee782f454a09310a28abe73653e6c82d06a79855))
* generate QPalette for wider Qt compatibility ([7bd2f5d](https://github.com/playtron-os/cosmic-settings-daemon/commit/7bd2f5d3c731fb56a572aa8bff7f87ca8e9603e6))
* manage sinks the mono sound setting ([9bbb910](https://github.com/playtron-os/cosmic-settings-daemon/commit/9bbb91063a90d3b590fcc2e52bdd51415b11aacc))
* notifications and sound alerts on AC hotplug and low battery ([bf2e505](https://github.com/playtron-os/cosmic-settings-daemon/commit/bf2e505e450ab092010fa60ba75a6d9e9a8539f1))
* power off system action ([243c36e](https://github.com/playtron-os/cosmic-settings-daemon/commit/243c36e9e66dcbec83fa63cb3adaceb3bea166a4))
* set gnome button layout when cosmic button layout changes ([1be0845](https://github.com/playtron-os/cosmic-settings-daemon/commit/1be0845ce73329dda34d76f00d666444d6ce66ed))
* set gnome gtk-theme and color-scheme on theme changes ([8b36191](https://github.com/playtron-os/cosmic-settings-daemon/commit/8b36191c6cf84f482784df40ead4661adb7f078c))
* set notification sounds to notification media role ([9111bf0](https://github.com/playtron-os/cosmic-settings-daemon/commit/9111bf0dfc65a4708a219d22616397819f6a6222))
* shift+alt+tab ([747e482](https://github.com/playtron-os/cosmic-settings-daemon/commit/747e482ca197497ee3bc5f6e9dcd23c73e592e47))
* **shortcuts:** touchpad toggl ([8b33437](https://github.com/playtron-os/cosmic-settings-daemon/commit/8b3343794fb572e86fb835ec3b81648d67502288))
* support external monitors using the ddc/ci crate ([93a0878](https://github.com/playtron-os/cosmic-settings-daemon/commit/93a0878076b8ebef4a1b84f0e00e81a40378ef15))
* support for external monitors using ddc-ci ([fa82bdf](https://github.com/playtron-os/cosmic-settings-daemon/commit/fa82bdf9fe7b5f5bd6008f32f393efd5e7a71c47))
* support Qt theme generation with qt5ct & qt6ct ([defa9f7](https://github.com/playtron-os/cosmic-settings-daemon/commit/defa9f790432c70054ca1f39737d879ada5d0252))
* sync with greeter state ([c6b0cbd](https://github.com/playtron-os/cosmic-settings-daemon/commit/c6b0cbd9523909322121eeca1b1f62277246da73))
* theme autoswitch support ([9813ae0](https://github.com/playtron-os/cosmic-settings-daemon/commit/9813ae08a29cbccc0cfe74efb5f7938843f6808d))
* trigger headset/headphone confirmation when availability is unknown ([da4f815](https://github.com/playtron-os/cosmic-settings-daemon/commit/da4f8150361575199fa5a13b2834efb7d7cbdcf2))
* use locally-cached timezone geopositions over geoclue ([f609f7d](https://github.com/playtron-os/cosmic-settings-daemon/commit/f609f7d342bf5fb87ecc3779b8173d3bcd013417))
