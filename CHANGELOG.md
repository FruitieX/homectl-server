# Changelog

## 0.1.0 (2025-08-25)


### ⚠ BREAKING CHANGES

* rename transition_ms to transition and update type to f32

### Features

* --dry-run CLI flag ([20f0c05](https://github.com/FruitieX/homectl-server/commit/20f0c05a43d2bcfda00f9cadcf26692c5af84dde))
* enhance scene cycling with optional device and group detection ([d61c71c](https://github.com/FruitieX/homectl-server/commit/d61c71cdd28b787b516257493cffed770be25910))
* improve performance by skipping db device update in some cases ([5c1bae8](https://github.com/FruitieX/homectl-server/commit/5c1bae83f0d2d64a3e8d151225fc2c40febb9d06))
* log which device/group keys scene was activated for ([05d4282](https://github.com/FruitieX/homectl-server/commit/05d4282f4e7b19d2e3821fe9570b34bbbc5fe859))
* **mqtt:** support setting device brightness range, power on/off values ([3871b77](https://github.com/FruitieX/homectl-server/commit/3871b77bfcd28c4bd9580c01c92d81be295de360))
* **mqtt:** update transition handling and add capabilities override ([6fcc598](https://github.com/FruitieX/homectl-server/commit/6fcc598e6fafbae3aa2b31aaef3510839fb9f29f))
* restore devices from db at startup ([e17ae9d](https://github.com/FruitieX/homectl-server/commit/e17ae9dee10e3de0beca7871550832cc06d7e5f8))
* scene state overrides ([8ee9c5a](https://github.com/FruitieX/homectl-server/commit/8ee9c5a7507780ed9a583d4e68e0af89fad026c6))
* store scene overrides in separate table ([3ad6026](https://github.com/FruitieX/homectl-server/commit/3ad6026233aa4e846321c19661faeb86438459f7))
* support multiple sensor value fields ([233a5f8](https://github.com/FruitieX/homectl-server/commit/233a5f8fe53f24c6cedfef16b82a91198b6057d5))
* ui state action ([cbc41bb](https://github.com/FruitieX/homectl-server/commit/cbc41bbf0ac321dc5335ba90789bb5529f84be5f))


### Bug Fixes

* apply scene data when restoring db device ([bed54ab](https://github.com/FruitieX/homectl-server/commit/bed54ab71506dc5da1f064abb6188acedd04fe24))
* avoid blocking async thread ([57e7ae3](https://github.com/FruitieX/homectl-server/commit/57e7ae343e2ea724549a277181878688207056bf))
* **deps:** pin rust crate clap to =4.5.30 ([#569](https://github.com/FruitieX/homectl-server/issues/569)) ([c9f3752](https://github.com/FruitieX/homectl-server/commit/c9f375245d20c61df0490b190a164932a6a1c564))
* **deps:** update rust crate async-trait to v0.1.78 ([0443b35](https://github.com/FruitieX/homectl-server/commit/0443b352c9df713b8db66063e158973a7c53b42a))
* **deps:** update rust crate async-trait to v0.1.79 ([9834234](https://github.com/FruitieX/homectl-server/commit/9834234162afba30db75bd8d13a98f9ab5581bf0))
* **deps:** update rust crate async-trait to v0.1.80 ([415c416](https://github.com/FruitieX/homectl-server/commit/415c416380d53088e6fecb9769359fa2ca058326))
* **deps:** update rust crate async-trait to v0.1.81 ([0e2adc1](https://github.com/FruitieX/homectl-server/commit/0e2adc15816549f34b3764f282d69778232636f4))
* **deps:** update rust crate async-trait to v0.1.82 ([160e62b](https://github.com/FruitieX/homectl-server/commit/160e62b03936015cc119514b47fe3e1897ffcf3c))
* **deps:** update rust crate async-trait to v0.1.83 ([f666dff](https://github.com/FruitieX/homectl-server/commit/f666dffb8f9da6781e449279731717127b562361))
* **deps:** update rust crate async-trait to v0.1.84 ([#529](https://github.com/FruitieX/homectl-server/issues/529)) ([554b7a3](https://github.com/FruitieX/homectl-server/commit/554b7a310a9e35ad7c2a4b84d8b9493e76c0b578))
* **deps:** update rust crate async-trait to v0.1.85 ([#532](https://github.com/FruitieX/homectl-server/issues/532)) ([5f30914](https://github.com/FruitieX/homectl-server/commit/5f309144aff8fd7cf2189368908796df8db37b6e))
* **deps:** update rust crate async-trait to v0.1.86 ([#552](https://github.com/FruitieX/homectl-server/issues/552)) ([d20a1ff](https://github.com/FruitieX/homectl-server/commit/d20a1ff40199d63d3fd65c01737be47fb9c2b212))
* **deps:** update rust crate async-trait to v0.1.87 ([#586](https://github.com/FruitieX/homectl-server/issues/586)) ([ed41d84](https://github.com/FruitieX/homectl-server/commit/ed41d84f7654f6edf7a13e5ade2af0153d00dfad))
* **deps:** update rust crate async-trait to v0.1.88 ([#599](https://github.com/FruitieX/homectl-server/issues/599)) ([98f16ed](https://github.com/FruitieX/homectl-server/commit/98f16ed46ecd985b9621e0a33b025d64be0da146))
* **deps:** update rust crate async-trait to v0.1.89 ([#698](https://github.com/FruitieX/homectl-server/issues/698)) ([a69c372](https://github.com/FruitieX/homectl-server/commit/a69c372f339756c3f5ac6ae90dac79859beea3b1))
* **deps:** update rust crate bytes to v1.10.0 ([#555](https://github.com/FruitieX/homectl-server/issues/555)) ([3decd30](https://github.com/FruitieX/homectl-server/commit/3decd302a5cfcdf73981a89e9e89520850cb02a4))
* **deps:** update rust crate bytes to v1.10.1 ([#589](https://github.com/FruitieX/homectl-server/issues/589)) ([b76686d](https://github.com/FruitieX/homectl-server/commit/b76686d9b08e1a2ebaffdb79a306e975678368a7))
* **deps:** update rust crate bytes to v1.6.0 ([9886886](https://github.com/FruitieX/homectl-server/commit/9886886171a62341d27326a2aba6b05becb8c5a6))
* **deps:** update rust crate bytes to v1.6.1 ([ed2c1bc](https://github.com/FruitieX/homectl-server/commit/ed2c1bc32f0cb2b3bbeb91a77446553ef3215900))
* **deps:** update rust crate bytes to v1.7.0 ([3b2d414](https://github.com/FruitieX/homectl-server/commit/3b2d414415f6e77184b8ca410a4ac30a4a789c0f))
* **deps:** update rust crate bytes to v1.7.1 ([b31e7ab](https://github.com/FruitieX/homectl-server/commit/b31e7ab9256829d734218b9976fd7fd0c34e7b8e))
* **deps:** update rust crate bytes to v1.7.2 ([65c1281](https://github.com/FruitieX/homectl-server/commit/65c128110c31622ec053e31f539744c2773013d0))
* **deps:** update rust crate bytes to v1.8.0 ([8c56087](https://github.com/FruitieX/homectl-server/commit/8c560878851588f2700280aeaa5fe135a4609672))
* **deps:** update rust crate bytes to v1.9.0 ([#503](https://github.com/FruitieX/homectl-server/issues/503)) ([cb86ad4](https://github.com/FruitieX/homectl-server/commit/cb86ad49586aa9d75ef5f81e18810f424fbca81c))
* **deps:** update rust crate chrono to v0.4.37 ([d00431e](https://github.com/FruitieX/homectl-server/commit/d00431eae37be6e388ee77592fd9060d36d64cbc))
* **deps:** update rust crate chrono to v0.4.38 ([cc310ab](https://github.com/FruitieX/homectl-server/commit/cc310ab14ea411e582d65e9d01f0afd382f5d75b))
* **deps:** update rust crate chrono to v0.4.39 ([#512](https://github.com/FruitieX/homectl-server/issues/512)) ([c689eea](https://github.com/FruitieX/homectl-server/commit/c689eea0f6da5ecf29a3ff02fe38f8e448003026))
* **deps:** update rust crate chrono to v0.4.40 ([#579](https://github.com/FruitieX/homectl-server/issues/579)) ([474b576](https://github.com/FruitieX/homectl-server/commit/474b5760db5e4997ec87fa1e47142f744c00c798))
* **deps:** update rust crate chrono to v0.4.41 ([#632](https://github.com/FruitieX/homectl-server/issues/632)) ([e87cdaa](https://github.com/FruitieX/homectl-server/commit/e87cdaacc6c1a61bb894bbc77b8472981225d4c0))
* **deps:** update rust crate clap to v4.5.31 ([#577](https://github.com/FruitieX/homectl-server/issues/577)) ([c7419aa](https://github.com/FruitieX/homectl-server/commit/c7419aaf131904174d60cdd1f107611f6540025f))
* **deps:** update rust crate clap to v4.5.32 ([#594](https://github.com/FruitieX/homectl-server/issues/594)) ([9c55011](https://github.com/FruitieX/homectl-server/commit/9c5501120a68b362a98d6aae1ea500f06434f69c))
* **deps:** update rust crate clap to v4.5.34 ([#609](https://github.com/FruitieX/homectl-server/issues/609)) ([f78f267](https://github.com/FruitieX/homectl-server/commit/f78f267ab35b2b345bfcf46bf4a666ce10799693))
* **deps:** update rust crate clap to v4.5.35 ([#614](https://github.com/FruitieX/homectl-server/issues/614)) ([44710e6](https://github.com/FruitieX/homectl-server/commit/44710e612fc84c080703300a079757df7b0753bf))
* **deps:** update rust crate clap to v4.5.36 ([#621](https://github.com/FruitieX/homectl-server/issues/621)) ([19933c9](https://github.com/FruitieX/homectl-server/commit/19933c9e157f67a5658c1bcbd8be53aaffcc9a0e))
* **deps:** update rust crate clap to v4.5.37 ([#625](https://github.com/FruitieX/homectl-server/issues/625)) ([c50c9b3](https://github.com/FruitieX/homectl-server/commit/c50c9b39fbd5b330b8ef241fc3719e6f84087747))
* **deps:** update rust crate clap to v4.5.38 ([#637](https://github.com/FruitieX/homectl-server/issues/637)) ([a6b061d](https://github.com/FruitieX/homectl-server/commit/a6b061d41f6c489897f4dfb97a6ea548e77a1aad))
* **deps:** update rust crate clap to v4.5.39 ([#647](https://github.com/FruitieX/homectl-server/issues/647)) ([2d06f9b](https://github.com/FruitieX/homectl-server/commit/2d06f9b448c0c231ac84779c1570508f63c6293b))
* **deps:** update rust crate clap to v4.5.40 ([#652](https://github.com/FruitieX/homectl-server/issues/652)) ([56ea0b7](https://github.com/FruitieX/homectl-server/commit/56ea0b774c264710493223a71874c4da3e5b07f3))
* **deps:** update rust crate clap to v4.5.41 ([#669](https://github.com/FruitieX/homectl-server/issues/669)) ([ea04d84](https://github.com/FruitieX/homectl-server/commit/ea04d849704527de36e35de8bfda9b5460c27a39))
* **deps:** update rust crate clap to v4.5.42 ([#679](https://github.com/FruitieX/homectl-server/issues/679)) ([f961c3f](https://github.com/FruitieX/homectl-server/commit/f961c3f1a6faed97446bb8fc09fa7333d6f23d3a))
* **deps:** update rust crate clap to v4.5.43 ([#689](https://github.com/FruitieX/homectl-server/issues/689)) ([b18a9c0](https://github.com/FruitieX/homectl-server/commit/b18a9c033f3f7c25fa632b0064b9042c97fefbbc))
* **deps:** update rust crate clap to v4.5.44 ([#693](https://github.com/FruitieX/homectl-server/issues/693)) ([c314ce4](https://github.com/FruitieX/homectl-server/commit/c314ce412952c2f49e37a5203b189b016776b733))
* **deps:** update rust crate clap to v4.5.45 ([#694](https://github.com/FruitieX/homectl-server/issues/694)) ([59cead7](https://github.com/FruitieX/homectl-server/commit/59cead71b48e065d968a9571541efc8d1a2faae4))
* **deps:** update rust crate color-eyre to v0.6.3 ([ac32494](https://github.com/FruitieX/homectl-server/commit/ac324944a5183ca1163181d35ca6b6a181ae2fc2))
* **deps:** update rust crate color-eyre to v0.6.5 ([#633](https://github.com/FruitieX/homectl-server/issues/633)) ([498ac01](https://github.com/FruitieX/homectl-server/commit/498ac01a1fda705432561a4a44d259a03e176906))
* **deps:** update rust crate config to v0.14.0 ([473f589](https://github.com/FruitieX/homectl-server/commit/473f589c38e3264b755fb9b8c0e6712550199f0b))
* **deps:** update rust crate config to v0.14.1 ([2d74e0d](https://github.com/FruitieX/homectl-server/commit/2d74e0d9cea33596884aa2d04987e8fa27fd5bf3))
* **deps:** update rust crate config to v0.15.0 ([#516](https://github.com/FruitieX/homectl-server/issues/516)) ([2c8dfa0](https://github.com/FruitieX/homectl-server/commit/2c8dfa063971ec6b941b381212f174fcc6bae491))
* **deps:** update rust crate config to v0.15.11 ([#595](https://github.com/FruitieX/homectl-server/issues/595)) ([530fa43](https://github.com/FruitieX/homectl-server/commit/530fa437eaae4b12d96b78103dad07f19e19b959))
* **deps:** update rust crate config to v0.15.13 ([#670](https://github.com/FruitieX/homectl-server/issues/670)) ([e05a220](https://github.com/FruitieX/homectl-server/commit/e05a220269cd9a39ec6a824d3624d210845a34b7))
* **deps:** update rust crate config to v0.15.14 ([#695](https://github.com/FruitieX/homectl-server/issues/695)) ([a682f27](https://github.com/FruitieX/homectl-server/commit/a682f2761d11da93ae5ba93a9bd87856c77db3a8))
* **deps:** update rust crate config to v0.15.3 ([#517](https://github.com/FruitieX/homectl-server/issues/517)) ([17d1138](https://github.com/FruitieX/homectl-server/commit/17d113810b5cba9ecfe74539af15f346e2e5ee96))
* **deps:** update rust crate config to v0.15.4 ([#519](https://github.com/FruitieX/homectl-server/issues/519)) ([365a67f](https://github.com/FruitieX/homectl-server/commit/365a67f9dba878b8302ddeb14b1b1e8641a88a37))
* **deps:** update rust crate config to v0.15.5 ([#536](https://github.com/FruitieX/homectl-server/issues/536)) ([1cc3493](https://github.com/FruitieX/homectl-server/commit/1cc349300e8a64f6d5ed6e7a5d67ad3281f886cd))
* **deps:** update rust crate config to v0.15.6 ([#541](https://github.com/FruitieX/homectl-server/issues/541)) ([630b62c](https://github.com/FruitieX/homectl-server/commit/630b62ce46faec06d4dcd44b32e2e3187117d9a3))
* **deps:** update rust crate config to v0.15.7 ([#551](https://github.com/FruitieX/homectl-server/issues/551)) ([b420247](https://github.com/FruitieX/homectl-server/commit/b420247ebbbfc1b03286c5d0ed0ca52f35921e0b))
* **deps:** update rust crate config to v0.15.8 ([#562](https://github.com/FruitieX/homectl-server/issues/562)) ([3f2e358](https://github.com/FruitieX/homectl-server/commit/3f2e358b6e6854c773ca9e955550bc151f667e35))
* **deps:** update rust crate config to v0.15.9 ([#588](https://github.com/FruitieX/homectl-server/issues/588)) ([0882e9f](https://github.com/FruitieX/homectl-server/commit/0882e9f08fb2fdf0c5bad51f622a293d37ec1b5a))
* **deps:** update rust crate croner to v2.0.5 ([f7dc53d](https://github.com/FruitieX/homectl-server/commit/f7dc53db8e6a99da44f6e144c94484d9e6e7ba9d))
* **deps:** update rust crate croner to v2.0.6 ([1e85671](https://github.com/FruitieX/homectl-server/commit/1e85671275868ef29bf170b5f480ccbe63739fd9))
* **deps:** update rust crate croner to v2.0.8 ([#497](https://github.com/FruitieX/homectl-server/issues/497)) ([d89a538](https://github.com/FruitieX/homectl-server/commit/d89a538c7a61db17c04d1146f8e1d91cae44453f))
* **deps:** update rust crate croner to v2.1.0 ([#499](https://github.com/FruitieX/homectl-server/issues/499)) ([e44e44c](https://github.com/FruitieX/homectl-server/commit/e44e44c5790713a4dac4c365cd2b1888b915f2ce))
* **deps:** update rust crate croner to v2.2.0 ([#659](https://github.com/FruitieX/homectl-server/issues/659)) ([6427113](https://github.com/FruitieX/homectl-server/commit/64271132335f044903cd748ce315e8c2ea58453d))
* **deps:** update rust crate itertools to v0.13.0 ([9ce931a](https://github.com/FruitieX/homectl-server/commit/9ce931a461a82af2542485f4b8456cae92a7a300))
* **deps:** update rust crate itertools to v0.14.0 ([#528](https://github.com/FruitieX/homectl-server/issues/528)) ([de35a74](https://github.com/FruitieX/homectl-server/commit/de35a74f8254c74250034572493baf8bf902dd28))
* **deps:** update rust crate jsonptr to v0.4.5 ([3bb9271](https://github.com/FruitieX/homectl-server/commit/3bb927179afc6888277d9e39542ddb9ed7985690))
* **deps:** update rust crate jsonptr to v0.4.6 ([2ca6342](https://github.com/FruitieX/homectl-server/commit/2ca6342bd538692996c78016de3d84024556c47f))
* **deps:** update rust crate jsonptr to v0.4.7 ([f5d8170](https://github.com/FruitieX/homectl-server/commit/f5d81705ad826b5bf60f5dcb88f04dd437e0755b))
* **deps:** update rust crate jsonptr to v0.5.1 ([376cf69](https://github.com/FruitieX/homectl-server/commit/376cf69f202500d955a97eb2389b95d0175366c3))
* **deps:** update rust crate jsonptr to v0.6.0 ([18cfbd5](https://github.com/FruitieX/homectl-server/commit/18cfbd5af56a0c5e2a137741de094d4faf3db8b5))
* **deps:** update rust crate jsonptr to v0.6.1 ([967d334](https://github.com/FruitieX/homectl-server/commit/967d334f6ae713d7abc5c50206f224f54c86a746))
* **deps:** update rust crate jsonptr to v0.6.2 ([0c3302a](https://github.com/FruitieX/homectl-server/commit/0c3302acf79a9ec9deab2c33653bd2c0ee29aea8))
* **deps:** update rust crate jsonptr to v0.6.3 ([e78b543](https://github.com/FruitieX/homectl-server/commit/e78b54345b4097244a42ad380b0892000bc3f1dd))
* **deps:** update rust crate jsonptr to v0.7.0 ([#564](https://github.com/FruitieX/homectl-server/issues/564)) ([236cb02](https://github.com/FruitieX/homectl-server/commit/236cb02b6cc6ab9345af64d7202fa7dc92b55dbe))
* **deps:** update rust crate jsonptr to v0.7.1 ([#565](https://github.com/FruitieX/homectl-server/issues/565)) ([e6476c6](https://github.com/FruitieX/homectl-server/commit/e6476c6253e5837f8bcb1cedb0cbd2fed436fce5))
* **deps:** update rust crate log to v0.4.21 ([8c253cd](https://github.com/FruitieX/homectl-server/commit/8c253cd9db13b58b736a2e9abc59ef91e07ba159))
* **deps:** update rust crate log to v0.4.22 ([9af6b94](https://github.com/FruitieX/homectl-server/commit/9af6b943862ea14384d3155944cc904649fccd19))
* **deps:** update rust crate log to v0.4.24 ([#537](https://github.com/FruitieX/homectl-server/issues/537)) ([3b709f4](https://github.com/FruitieX/homectl-server/commit/3b709f47a35e7128a46f68cdf799139dbb2fca1f))
* **deps:** update rust crate log to v0.4.25 ([#540](https://github.com/FruitieX/homectl-server/issues/540)) ([b41616b](https://github.com/FruitieX/homectl-server/commit/b41616b05b5df766676aae7a413496b3f36f446e))
* **deps:** update rust crate log to v0.4.26 ([#574](https://github.com/FruitieX/homectl-server/issues/574)) ([f0d5073](https://github.com/FruitieX/homectl-server/commit/f0d50734cff16f75fd830f683cae5724ff79ff41))
* **deps:** update rust crate log to v0.4.27 ([#608](https://github.com/FruitieX/homectl-server/issues/608)) ([0b57d4a](https://github.com/FruitieX/homectl-server/commit/0b57d4a84174b53d7f58a91593f478161cf9570b))
* **deps:** update rust crate once_cell to v1.20.0 ([87b1e6f](https://github.com/FruitieX/homectl-server/commit/87b1e6fa2e59a4abb5e2a98c091bdd2eff1e8f96))
* **deps:** update rust crate once_cell to v1.20.1 ([c111324](https://github.com/FruitieX/homectl-server/commit/c111324adb9250617c52545cdd01b9e71c485bae))
* **deps:** update rust crate once_cell to v1.20.2 ([8d5f6a1](https://github.com/FruitieX/homectl-server/commit/8d5f6a10e3b3fb7bd22045ede30de53a1e7336e5))
* **deps:** update rust crate once_cell to v1.20.3 ([#560](https://github.com/FruitieX/homectl-server/issues/560)) ([ba571b4](https://github.com/FruitieX/homectl-server/commit/ba571b4c0dbd7df722c7f9117c0341b97ea8e8d8))
* **deps:** update rust crate once_cell to v1.21.0 ([#593](https://github.com/FruitieX/homectl-server/issues/593)) ([f0bb750](https://github.com/FruitieX/homectl-server/commit/f0bb750350c4d2a3aaaba876c980537e7b0174e4))
* **deps:** update rust crate once_cell to v1.21.1 ([#597](https://github.com/FruitieX/homectl-server/issues/597)) ([2266c7f](https://github.com/FruitieX/homectl-server/commit/2266c7f44da933ce49444077650ab09a66f9763b))
* **deps:** update rust crate once_cell to v1.21.2 ([#610](https://github.com/FruitieX/homectl-server/issues/610)) ([6c08e4d](https://github.com/FruitieX/homectl-server/commit/6c08e4d2378d1b0f2902d9e17732be1dcc697924))
* **deps:** update rust crate once_cell to v1.21.3 ([#611](https://github.com/FruitieX/homectl-server/issues/611)) ([3450c3e](https://github.com/FruitieX/homectl-server/commit/3450c3e7e913dcf7bfb0db353df3132be000f346))
* **deps:** update rust crate ordered-float to v4.2.1 ([6553962](https://github.com/FruitieX/homectl-server/commit/6553962b90772fe1d67946b81f72bd4421dfe0c6))
* **deps:** update rust crate ordered-float to v4.2.2 ([181105a](https://github.com/FruitieX/homectl-server/commit/181105a9de4f8b88e74f51c1d0b5fed8c1771510))
* **deps:** update rust crate ordered-float to v4.3.0 ([90d64fa](https://github.com/FruitieX/homectl-server/commit/90d64fa09011f3b03932fb630b37e99250200e65))
* **deps:** update rust crate ordered-float to v4.4.0 ([0f417e3](https://github.com/FruitieX/homectl-server/commit/0f417e34f482b521bf93381a917cb1fa1ad4ebc2))
* **deps:** update rust crate ordered-float to v4.5.0 ([704d14d](https://github.com/FruitieX/homectl-server/commit/704d14db182fbc79bbf8c178108c30afc1400358))
* **deps:** update rust crate ordered-float to v4.6.0 ([#518](https://github.com/FruitieX/homectl-server/issues/518)) ([fb0c566](https://github.com/FruitieX/homectl-server/commit/fb0c566429477057bd12a16703e1ccd21734cf6a))
* **deps:** update rust crate ordered-float to v5 ([#567](https://github.com/FruitieX/homectl-server/issues/567)) ([0fd4aa0](https://github.com/FruitieX/homectl-server/commit/0fd4aa05fa03ef2c9f2a028a08ec8dd51c5483e6))
* **deps:** update rust crate palette to v0.7.5 ([3fc62b2](https://github.com/FruitieX/homectl-server/commit/3fc62b28e2d45a1f56b7b7bca56834cf3c2044ba))
* **deps:** update rust crate palette to v0.7.6 ([6acebee](https://github.com/FruitieX/homectl-server/commit/6acebee1c5b2c8e69458cadfc1ee35124c485a88))
* **deps:** update rust crate rumqttc to v0.24.0 ([52d886f](https://github.com/FruitieX/homectl-server/commit/52d886f19ebfa77dc0ca3feccd880b9846f6ee33))
* **deps:** update rust crate serde to v1.0.197 ([40fade7](https://github.com/FruitieX/homectl-server/commit/40fade720d4690af1bed86ce3edd030e679aeeae))
* **deps:** update rust crate serde to v1.0.198 ([a3ff611](https://github.com/FruitieX/homectl-server/commit/a3ff611151028ec6f06c26c1387baef10697d4af))
* **deps:** update rust crate serde to v1.0.199 ([75bcfd4](https://github.com/FruitieX/homectl-server/commit/75bcfd469152b0cbc6806dcb5524d86caf409eac))
* **deps:** update rust crate serde to v1.0.200 ([e9180f9](https://github.com/FruitieX/homectl-server/commit/e9180f9d18969a4dbc966c65f98b913e4b55c29c))
* **deps:** update rust crate serde to v1.0.201 ([6dab1ee](https://github.com/FruitieX/homectl-server/commit/6dab1ee08af4a9d2fb43af9eda63d270c982d630))
* **deps:** update rust crate serde to v1.0.202 ([9c68644](https://github.com/FruitieX/homectl-server/commit/9c686441ba9b9fa0f2594841ef8e5ad4140b8470))
* **deps:** update rust crate serde to v1.0.203 ([d0a37ec](https://github.com/FruitieX/homectl-server/commit/d0a37ec890ef3eec1c4d8dea9fc4bac4ebe7ebdf))
* **deps:** update rust crate serde to v1.0.204 ([fb6be70](https://github.com/FruitieX/homectl-server/commit/fb6be70c3c7df2b9d2683a21158f13d4489b6dcc))
* **deps:** update rust crate serde to v1.0.205 ([efe543b](https://github.com/FruitieX/homectl-server/commit/efe543b26e6218a1fea33f33bb44d0c2799d3dab))
* **deps:** update rust crate serde to v1.0.206 ([ac30e81](https://github.com/FruitieX/homectl-server/commit/ac30e8157ada1b008a500c6720888a5f00cff9c4))
* **deps:** update rust crate serde to v1.0.207 ([92b1300](https://github.com/FruitieX/homectl-server/commit/92b130033764fc050cc665b8e0423e8fb2341754))
* **deps:** update rust crate serde to v1.0.208 ([5018765](https://github.com/FruitieX/homectl-server/commit/5018765d85c97917bd0391e999d02dc631be7c9d))
* **deps:** update rust crate serde to v1.0.209 ([78f621b](https://github.com/FruitieX/homectl-server/commit/78f621bb5a0d557e1b10869271e10ad80ba36f53))
* **deps:** update rust crate serde to v1.0.210 ([8d0a72e](https://github.com/FruitieX/homectl-server/commit/8d0a72e92b7d3e8fb3a9b6aa88bef37093889791))
* **deps:** update rust crate serde to v1.0.211 ([f7c89cd](https://github.com/FruitieX/homectl-server/commit/f7c89cd84049fa36422a4b59abb13fe9b5ebe42b))
* **deps:** update rust crate serde to v1.0.213 ([8cd8680](https://github.com/FruitieX/homectl-server/commit/8cd86805b64434d2de70d1088cf2c71a25cedd1d))
* **deps:** update rust crate serde to v1.0.214 ([2630d3a](https://github.com/FruitieX/homectl-server/commit/2630d3a15035268ccea0c2a20e6ee01cc8703817))
* **deps:** update rust crate serde to v1.0.215 ([#490](https://github.com/FruitieX/homectl-server/issues/490)) ([854d554](https://github.com/FruitieX/homectl-server/commit/854d554025a2a724675cf1317976d1e267958146))
* **deps:** update rust crate serde to v1.0.216 ([#513](https://github.com/FruitieX/homectl-server/issues/513)) ([ae3db2a](https://github.com/FruitieX/homectl-server/commit/ae3db2a1102b15f1c9de25f197090fca0038897b))
* **deps:** update rust crate serde to v1.0.217 ([#524](https://github.com/FruitieX/homectl-server/issues/524)) ([1845577](https://github.com/FruitieX/homectl-server/commit/184557773608857edbe9a388ec0439885c620c30))
* **deps:** update rust crate serde to v1.0.218 ([#571](https://github.com/FruitieX/homectl-server/issues/571)) ([0e2dc27](https://github.com/FruitieX/homectl-server/commit/0e2dc2709c8105510fec2846667e53788c9476f6))
* **deps:** update rust crate serde to v1.0.219 ([#591](https://github.com/FruitieX/homectl-server/issues/591)) ([5fbf00f](https://github.com/FruitieX/homectl-server/commit/5fbf00f38d7afc98a598f579a8f455d752fa1420))
* **deps:** update rust crate serde_json to v1.0.114 ([842f156](https://github.com/FruitieX/homectl-server/commit/842f156d9f4da25ff5fd2f85714eb514ac929265))
* **deps:** update rust crate serde_json to v1.0.115 ([70bf768](https://github.com/FruitieX/homectl-server/commit/70bf768ef105fbc346090f07c6630a35d3ce3d96))
* **deps:** update rust crate serde_json to v1.0.116 ([b7f40f6](https://github.com/FruitieX/homectl-server/commit/b7f40f692c6e26eae7beb14409cd5446ea3275e3))
* **deps:** update rust crate serde_json to v1.0.117 ([ab01f75](https://github.com/FruitieX/homectl-server/commit/ab01f75647adf2371c47ec282fce08a7b9dad73c))
* **deps:** update rust crate serde_json to v1.0.118 ([635d804](https://github.com/FruitieX/homectl-server/commit/635d804f1c2114887c03c51ddf231a5e6bc6e961))
* **deps:** update rust crate serde_json to v1.0.119 ([12bfd02](https://github.com/FruitieX/homectl-server/commit/12bfd02bf2970b48fab47f17772733d3cb7a687f))
* **deps:** update rust crate serde_json to v1.0.120 ([6a543c7](https://github.com/FruitieX/homectl-server/commit/6a543c7eedabdaeec8026c0425a148587924f1a6))
* **deps:** update rust crate serde_json to v1.0.121 ([647c5f5](https://github.com/FruitieX/homectl-server/commit/647c5f53c436b6fc0399f48bcac1bdb47c83a812))
* **deps:** update rust crate serde_json to v1.0.122 ([e265a1b](https://github.com/FruitieX/homectl-server/commit/e265a1bbd3b7598a9fdc6c730f5cae7f72516dce))
* **deps:** update rust crate serde_json to v1.0.124 ([306923f](https://github.com/FruitieX/homectl-server/commit/306923f4ef1c41545cb8160586920265a918bba3))
* **deps:** update rust crate serde_json to v1.0.125 ([92a5042](https://github.com/FruitieX/homectl-server/commit/92a5042ce42b688060542eda010ff9d0655c0413))
* **deps:** update rust crate serde_json to v1.0.127 ([58a2a86](https://github.com/FruitieX/homectl-server/commit/58a2a8694c7acae37c9e91519b9e27d05870eb8f))
* **deps:** update rust crate serde_json to v1.0.128 ([befccf7](https://github.com/FruitieX/homectl-server/commit/befccf76ae69f4cd0c3ef0bbb25ac4314b6887e6))
* **deps:** update rust crate serde_json to v1.0.129 ([529ee92](https://github.com/FruitieX/homectl-server/commit/529ee92f199f1635b67bf5e81d5e2a76ed06a80a))
* **deps:** update rust crate serde_json to v1.0.131 ([59f0fc1](https://github.com/FruitieX/homectl-server/commit/59f0fc1277b7e848c8736417899e4e8946fd120b))
* **deps:** update rust crate serde_json to v1.0.132 ([eb37d10](https://github.com/FruitieX/homectl-server/commit/eb37d1018f87a0cc1f3b8a1d7e9bfea155dcce01))
* **deps:** update rust crate serde_json to v1.0.133 ([#494](https://github.com/FruitieX/homectl-server/issues/494)) ([cbc5396](https://github.com/FruitieX/homectl-server/commit/cbc5396dece7dae6d340be399b2ea329a3df315b))
* **deps:** update rust crate serde_json to v1.0.134 ([#520](https://github.com/FruitieX/homectl-server/issues/520)) ([1d91840](https://github.com/FruitieX/homectl-server/commit/1d91840aff462f0033210ae5486205aafd371a97))
* **deps:** update rust crate serde_json to v1.0.135 ([#533](https://github.com/FruitieX/homectl-server/issues/533)) ([522cf17](https://github.com/FruitieX/homectl-server/commit/522cf17c75537ddf00987ae63e9e6a528a2c1b33))
* **deps:** update rust crate serde_json to v1.0.136 ([#543](https://github.com/FruitieX/homectl-server/issues/543)) ([d4c550b](https://github.com/FruitieX/homectl-server/commit/d4c550b4463fabecc220f6cfe1c1df447fdfe3d0))
* **deps:** update rust crate serde_json to v1.0.137 ([#544](https://github.com/FruitieX/homectl-server/issues/544)) ([c70d8b5](https://github.com/FruitieX/homectl-server/commit/c70d8b5e1c780e3b720485271581bdfa14c9a08b))
* **deps:** update rust crate serde_json to v1.0.138 ([#550](https://github.com/FruitieX/homectl-server/issues/550)) ([46edb73](https://github.com/FruitieX/homectl-server/commit/46edb735b319d08508008df0a7d7afd8fcd3e8e3))
* **deps:** update rust crate serde_json to v1.0.139 ([#572](https://github.com/FruitieX/homectl-server/issues/572)) ([07d2c1e](https://github.com/FruitieX/homectl-server/commit/07d2c1e801dd5222b39e1b6fce3f1ad44801d5db))
* **deps:** update rust crate serde_json to v1.0.140 ([#587](https://github.com/FruitieX/homectl-server/issues/587)) ([eea55d2](https://github.com/FruitieX/homectl-server/commit/eea55d21f959de22baf2901078d7ea83ce4a2183))
* **deps:** update rust crate serde_json to v1.0.141 ([#671](https://github.com/FruitieX/homectl-server/issues/671)) ([19e809c](https://github.com/FruitieX/homectl-server/commit/19e809c73b4b18a5943762fb1075d7baf191df9c))
* **deps:** update rust crate serde_json to v1.0.142 ([#681](https://github.com/FruitieX/homectl-server/issues/681)) ([787284b](https://github.com/FruitieX/homectl-server/commit/787284b41cdf2433e6ce88e08e923d5e1be170f6))
* **deps:** update rust crate serde_json to v1.0.143 ([#702](https://github.com/FruitieX/homectl-server/issues/702)) ([95cf2f8](https://github.com/FruitieX/homectl-server/commit/95cf2f81d0f5197b11b02529f49bec8a70c2f3d6))
* **deps:** update rust crate serde_json_path to v0.6.6 ([bd10ce5](https://github.com/FruitieX/homectl-server/commit/bd10ce50bba6c71968c89d79cc926016329c5e43))
* **deps:** update rust crate serde_json_path to v0.6.7 ([74f0c29](https://github.com/FruitieX/homectl-server/commit/74f0c29b30b889aa028b98dd9ef12bb40bb47e10))
* **deps:** update rust crate serde_json_path to v0.7.1 ([4818fb2](https://github.com/FruitieX/homectl-server/commit/4818fb23b8b839f790edb831df45e010725afad5))
* **deps:** update rust crate serde_json_path to v0.7.2 ([#554](https://github.com/FruitieX/homectl-server/issues/554)) ([a23d4e4](https://github.com/FruitieX/homectl-server/commit/a23d4e439200dfef0a62d087408956de0a613a66))
* **deps:** update rust crate serde_path_to_error to v0.1.16 ([735e147](https://github.com/FruitieX/homectl-server/commit/735e1471f399bdbd8bb824026e55aae8f67bda53))
* **deps:** update rust crate serde_path_to_error to v0.1.17 ([#585](https://github.com/FruitieX/homectl-server/issues/585)) ([9a1b2a9](https://github.com/FruitieX/homectl-server/commit/9a1b2a95ceab3b7fda556bad08d9478a8a1a008e))
* **deps:** update rust crate serde-this-or-that to v0.5.0 ([#547](https://github.com/FruitieX/homectl-server/issues/547)) ([a653a13](https://github.com/FruitieX/homectl-server/commit/a653a134043c2d63aa390a92c9641fe6db78c223))
* **deps:** update rust crate sqlx to v0.7.4 ([5f692b8](https://github.com/FruitieX/homectl-server/commit/5f692b8c81b8929ec3e58bb18e5e2697c8a890e0))
* **deps:** update rust crate sqlx to v0.8.0 ([d7af8fb](https://github.com/FruitieX/homectl-server/commit/d7af8fba83df571b00fe704010d51a1fc621cb23))
* **deps:** update rust crate sqlx to v0.8.1 [security] ([c3bcb83](https://github.com/FruitieX/homectl-server/commit/c3bcb83d43069e7e74b74d33176eee42a28afd74))
* **deps:** update rust crate sqlx to v0.8.2 ([be7f5a6](https://github.com/FruitieX/homectl-server/commit/be7f5a64852840c6da9cd2b794d4722419987d39))
* **deps:** update rust crate sqlx to v0.8.3 ([#530](https://github.com/FruitieX/homectl-server/issues/530)) ([4a9e8ef](https://github.com/FruitieX/homectl-server/commit/4a9e8ef7ec583de6f9cb6f4528f178cf89849cc9))
* **deps:** update rust crate sqlx to v0.8.4 ([#623](https://github.com/FruitieX/homectl-server/issues/623)) ([05ec032](https://github.com/FruitieX/homectl-server/commit/05ec0322e610691472af9fa8a6ed12310734cc8d))
* **deps:** update rust crate sqlx to v0.8.5 ([#624](https://github.com/FruitieX/homectl-server/issues/624)) ([6b22596](https://github.com/FruitieX/homectl-server/commit/6b2259616c6bcad88ec7ccf8969e4da677d663bc))
* **deps:** update rust crate sqlx to v0.8.6 ([#641](https://github.com/FruitieX/homectl-server/issues/641)) ([0906e3a](https://github.com/FruitieX/homectl-server/commit/0906e3ae11c65da8bcfbffb5ee45b1d3fc4af54c))
* **deps:** update rust crate tokio to v1.37.0 ([8816bda](https://github.com/FruitieX/homectl-server/commit/8816bdabad7a8f1121aeb401cd76133c60b47953))
* **deps:** update rust crate tokio to v1.38.0 ([7b5c620](https://github.com/FruitieX/homectl-server/commit/7b5c620a3036b91e81f863c312d717f8143e1e3c))
* **deps:** update rust crate tokio to v1.38.1 ([af48927](https://github.com/FruitieX/homectl-server/commit/af489274eedfcf8554341db3f7651be4388f9c39))
* **deps:** update rust crate tokio to v1.39.0 ([53ba473](https://github.com/FruitieX/homectl-server/commit/53ba473eef5e58e73db75f1ad10518bdf6cd0db7))
* **deps:** update rust crate tokio to v1.39.1 ([2c0a46e](https://github.com/FruitieX/homectl-server/commit/2c0a46e676cdbd862da77c0cb9be354797d736dc))
* **deps:** update rust crate tokio to v1.39.2 ([9a62cfb](https://github.com/FruitieX/homectl-server/commit/9a62cfb48ca55c82addeede5b7f551efb7ebd1f0))
* **deps:** update rust crate tokio to v1.39.3 ([7bb050e](https://github.com/FruitieX/homectl-server/commit/7bb050e782785370adb1e1f599df152ce9e1d3a6))
* **deps:** update rust crate tokio to v1.40.0 ([bd10b8f](https://github.com/FruitieX/homectl-server/commit/bd10b8f452ac452a59b7813f3ab716bcd7074fe7))
* **deps:** update rust crate tokio to v1.41.0 ([4efd498](https://github.com/FruitieX/homectl-server/commit/4efd49831eaba9ef41934f3b2344d3c2ed347f91))
* **deps:** update rust crate tokio to v1.41.1 ([#488](https://github.com/FruitieX/homectl-server/issues/488)) ([89319ad](https://github.com/FruitieX/homectl-server/commit/89319adcb9a01dbc49e50a362725d18d30e9e5b4))
* **deps:** update rust crate tokio to v1.42.0 ([#509](https://github.com/FruitieX/homectl-server/issues/509)) ([d912dc1](https://github.com/FruitieX/homectl-server/commit/d912dc1a0f18a7004696014984b2138760e598d2))
* **deps:** update rust crate tokio to v1.43.0 ([#535](https://github.com/FruitieX/homectl-server/issues/535)) ([749067f](https://github.com/FruitieX/homectl-server/commit/749067f403d5bbb20a470435de5a6af309093a26))
* **deps:** update rust crate tokio to v1.44.0 ([#590](https://github.com/FruitieX/homectl-server/issues/590)) ([8968c3f](https://github.com/FruitieX/homectl-server/commit/8968c3f1b242e597f9500a65edcfa2be38a72cc6))
* **deps:** update rust crate tokio to v1.44.1 ([#596](https://github.com/FruitieX/homectl-server/issues/596)) ([c49d997](https://github.com/FruitieX/homectl-server/commit/c49d9970afe5fca1d6d69d477c46802b9cc917c6))
* **deps:** update rust crate tokio to v1.44.2 ([#616](https://github.com/FruitieX/homectl-server/issues/616)) ([801ae07](https://github.com/FruitieX/homectl-server/commit/801ae07e4e8a3b63525f495871373bfb2b01db72))
* **deps:** update rust crate tokio to v1.45.0 ([#635](https://github.com/FruitieX/homectl-server/issues/635)) ([062096f](https://github.com/FruitieX/homectl-server/commit/062096f5d15b81a798f14d35ed2e0f135da71998))
* **deps:** update rust crate tokio to v1.45.1 ([#645](https://github.com/FruitieX/homectl-server/issues/645)) ([64ae77f](https://github.com/FruitieX/homectl-server/commit/64ae77f86d87d0d6d783fa593f6e5b99eb62ddac))
* **deps:** update rust crate tokio to v1.46.0 ([#666](https://github.com/FruitieX/homectl-server/issues/666)) ([341ca91](https://github.com/FruitieX/homectl-server/commit/341ca913243e39646dd759f02258e5652946bc41))
* **deps:** update rust crate tokio to v1.46.1 ([#667](https://github.com/FruitieX/homectl-server/issues/667)) ([acf1402](https://github.com/FruitieX/homectl-server/commit/acf1402f4baa5ed8b0fe77ac7f1513ca9cf38a99))
* **deps:** update rust crate tokio to v1.47.0 ([#676](https://github.com/FruitieX/homectl-server/issues/676)) ([24848fe](https://github.com/FruitieX/homectl-server/commit/24848feae0ca39775dbe7000fdb7bfdddb10c40f))
* **deps:** update rust crate tokio to v1.47.1 ([#683](https://github.com/FruitieX/homectl-server/issues/683)) ([13ced64](https://github.com/FruitieX/homectl-server/commit/13ced64d24d0bc2a60f1c95bbc7f64c227e73695))
* **deps:** update rust crate tokio-stream to v0.1.15 ([1ad0680](https://github.com/FruitieX/homectl-server/commit/1ad06801542907e62f5ce6af78600f2f2bcbd972))
* **deps:** update rust crate tokio-stream to v0.1.16 ([436dfc3](https://github.com/FruitieX/homectl-server/commit/436dfc3c1a30ac628d475f9edfb23a5fcf6d0cd3))
* **deps:** update rust crate tokio-stream to v0.1.17 ([#510](https://github.com/FruitieX/homectl-server/issues/510)) ([f2ee51f](https://github.com/FruitieX/homectl-server/commit/f2ee51f8913f827adb88c640087e18770ed2448d))
* **deps:** update rust crate toml to v0.8.11 ([3114c50](https://github.com/FruitieX/homectl-server/commit/3114c506168e8791db9c35adc338d6a512539ff2))
* **deps:** update rust crate toml to v0.8.12 ([5f8cc7d](https://github.com/FruitieX/homectl-server/commit/5f8cc7d4651a9abf519ab674f69c67c449de99e1))
* **deps:** update rust crate toml to v0.8.13 ([74989ae](https://github.com/FruitieX/homectl-server/commit/74989ae39421195ef162af232df22cd3b8a87a1d))
* **deps:** update rust crate toml to v0.8.14 ([d82ef95](https://github.com/FruitieX/homectl-server/commit/d82ef951834e83de0edfb95c28baeb591930c0c1))
* **deps:** update rust crate toml to v0.8.15 ([501a384](https://github.com/FruitieX/homectl-server/commit/501a3841f695478e5796d2d5a4ba78fd1d79d30c))
* **deps:** update rust crate toml to v0.8.16 ([89d033f](https://github.com/FruitieX/homectl-server/commit/89d033f960611c07396853f4406845bf517da91a))
* **deps:** update rust crate toml to v0.8.17 ([3cc8799](https://github.com/FruitieX/homectl-server/commit/3cc87996f79ce714884e6f0e46bea643dc673c93))
* **deps:** update rust crate toml to v0.8.19 ([09f69b4](https://github.com/FruitieX/homectl-server/commit/09f69b468a4bd485f023a38b1cee7e247e2078f3))
* **deps:** update rust crate toml to v0.8.20 ([#558](https://github.com/FruitieX/homectl-server/issues/558)) ([00a1435](https://github.com/FruitieX/homectl-server/commit/00a14357d43c12a75c7656f047479255f6ca934e))
* **deps:** update rust crate toml to v0.8.21 ([#628](https://github.com/FruitieX/homectl-server/issues/628)) ([4f31999](https://github.com/FruitieX/homectl-server/commit/4f319994af4662156f071247e8c549e6ba3dabb4))
* **deps:** update rust crate toml to v0.8.22 ([#630](https://github.com/FruitieX/homectl-server/issues/630)) ([cc24fab](https://github.com/FruitieX/homectl-server/commit/cc24fabc5a088c97705ca23adae08661147e2cfe))
* **deps:** update rust crate toml to v0.8.23 ([#650](https://github.com/FruitieX/homectl-server/issues/650)) ([a746ba7](https://github.com/FruitieX/homectl-server/commit/a746ba75a2ad3c3bb67d996682318994480bfb31))
* **deps:** update rust crate toml to v0.9.2 ([#672](https://github.com/FruitieX/homectl-server/issues/672)) ([a25a936](https://github.com/FruitieX/homectl-server/commit/a25a936da95982279f292f7642f15d661ac76f78))
* **deps:** update rust crate toml to v0.9.3 ([#678](https://github.com/FruitieX/homectl-server/issues/678)) ([9363d91](https://github.com/FruitieX/homectl-server/commit/9363d915f6b9084ab0618eccf569a942fdeeeb41))
* **deps:** update rust crate toml to v0.9.4 ([#680](https://github.com/FruitieX/homectl-server/issues/680)) ([666a8b1](https://github.com/FruitieX/homectl-server/commit/666a8b1665abeb4ead206de99510eafb8ac6fc79))
* **deps:** update rust crate toml to v0.9.5 ([#687](https://github.com/FruitieX/homectl-server/issues/687)) ([5011508](https://github.com/FruitieX/homectl-server/commit/5011508f7296fc7cf99976741b0d76a6b541edb9))
* **deps:** update rust crate ts-rs to v10 ([e2578c6](https://github.com/FruitieX/homectl-server/commit/e2578c6a134fcd65a0498fb0a390647c1331bd5a))
* **deps:** update rust crate ts-rs to v10.1.0 ([#505](https://github.com/FruitieX/homectl-server/issues/505)) ([5df8307](https://github.com/FruitieX/homectl-server/commit/5df830759c7ceb56af0dd0c3726fbb3a5e7555d8))
* **deps:** update rust crate ts-rs to v11 ([#648](https://github.com/FruitieX/homectl-server/issues/648)) ([38fb259](https://github.com/FruitieX/homectl-server/commit/38fb259ffc17ed66fc30583b574fa2988cc770c7))
* **deps:** update rust crate ts-rs to v11.0.1 ([#649](https://github.com/FruitieX/homectl-server/issues/649)) ([8f8a73f](https://github.com/FruitieX/homectl-server/commit/8f8a73ffe724d88cc00cfa9f5e84910638a3a4ae))
* **deps:** update rust crate ts-rs to v8 ([98dbbe3](https://github.com/FruitieX/homectl-server/commit/98dbbe38121e930f88f76d29714aed71fe4d88f4))
* **deps:** update rust crate ts-rs to v8.1.0 ([7170d5f](https://github.com/FruitieX/homectl-server/commit/7170d5f5ad0a894fd9f172fb3ea9b799b1b042d0))
* **deps:** update rust crate ts-rs to v9 ([c94f137](https://github.com/FruitieX/homectl-server/commit/c94f137171baa30f88326258c8cdd75f69894380))
* **deps:** update rust crate ts-rs to v9.0.1 ([88e84da](https://github.com/FruitieX/homectl-server/commit/88e84dab1b9b64f42fb53289718bbe6453981fe0))
* **deps:** update rust crate warp to v0.3.7 ([d024dbe](https://github.com/FruitieX/homectl-server/commit/d024dbeb7bc55c8d9bd461dadfd3c51788e683a5))
* **deps:** update rust-futures monorepo to v0.3.31 ([dd02765](https://github.com/FruitieX/homectl-server/commit/dd02765c83b11aa5824133bfb76b5c9a9f4532fc))
* don't skip writing updated device scene state to db ([f2c81d6](https://github.com/FruitieX/homectl-server/commit/f2c81d664b5a0c6343165d4396cba7eb1a22ab85))
* fix not being able to set color after turning on light ([1d0d70c](https://github.com/FruitieX/homectl-server/commit/1d0d70c77f2ec30db4b8e9783980d7dd49919d55))
* fix scene list not refreshing after edits ([e027b2a](https://github.com/FruitieX/homectl-server/commit/e027b2a430ab7e04cc6824118c7c86c3b7575d2d))
* fixes to typescript bindings ([38985c4](https://github.com/FruitieX/homectl-server/commit/38985c4dc3a1bcfcd8242eb00c92327ccc175efe))
* ignore MQTT messages with missing fields ([d3c2fbe](https://github.com/FruitieX/homectl-server/commit/d3c2fbefc647ed0a1c65a9835a4f74a2787f52af))
* insert restored db devices into keys_by_name map ([083b1d7](https://github.com/FruitieX/homectl-server/commit/083b1d7bc3bed0ecd7a48f8b29c388cebd234c16))
* make example config run ([cbecd77](https://github.com/FruitieX/homectl-server/commit/cbecd77c62498ea4816978296d68a282fb6392a8))
* scene overrides adhere to device/group filters ([59a4864](https://github.com/FruitieX/homectl-server/commit/59a4864abc0023e7447ec8950639a086782aff27))
* skip first external update after restoring device from db ([43798aa](https://github.com/FruitieX/homectl-server/commit/43798aa13492b3507545ceeb03bf745c31f44f4c))
* use db state when discovering device ([a089882](https://github.com/FruitieX/homectl-server/commit/a0898822f90388be2a1147d70b580c0957e60d9b))


### Code Refactoring

* rename transition_ms to transition and update type to f32 ([20ec4ae](https://github.com/FruitieX/homectl-server/commit/20ec4ae4ed69ce1f6c9a0799305cd1825c6e902c))

## [0.9.5](https://github.com/FruitieX/homectl-server/compare/v0.9.4...v0.9.5) (2024-07-23)


### Bug Fixes

* fix scene list not refreshing after edits ([e027b2a](https://github.com/FruitieX/homectl-server/commit/e027b2a430ab7e04cc6824118c7c86c3b7575d2d))

## [0.9.4](https://github.com/FruitieX/homectl-server/compare/v0.9.3...v0.9.4) (2024-07-23)


### Bug Fixes

* **deps:** update rust crate async-trait to v0.1.78 ([0443b35](https://github.com/FruitieX/homectl-server/commit/0443b352c9df713b8db66063e158973a7c53b42a))
* **deps:** update rust crate async-trait to v0.1.79 ([9834234](https://github.com/FruitieX/homectl-server/commit/9834234162afba30db75bd8d13a98f9ab5581bf0))
* **deps:** update rust crate async-trait to v0.1.80 ([415c416](https://github.com/FruitieX/homectl-server/commit/415c416380d53088e6fecb9769359fa2ca058326))
* **deps:** update rust crate async-trait to v0.1.81 ([0e2adc1](https://github.com/FruitieX/homectl-server/commit/0e2adc15816549f34b3764f282d69778232636f4))
* **deps:** update rust crate bytes to v1.6.0 ([9886886](https://github.com/FruitieX/homectl-server/commit/9886886171a62341d27326a2aba6b05becb8c5a6))
* **deps:** update rust crate bytes to v1.6.1 ([ed2c1bc](https://github.com/FruitieX/homectl-server/commit/ed2c1bc32f0cb2b3bbeb91a77446553ef3215900))
* **deps:** update rust crate chrono to v0.4.37 ([d00431e](https://github.com/FruitieX/homectl-server/commit/d00431eae37be6e388ee77592fd9060d36d64cbc))
* **deps:** update rust crate chrono to v0.4.38 ([cc310ab](https://github.com/FruitieX/homectl-server/commit/cc310ab14ea411e582d65e9d01f0afd382f5d75b))
* **deps:** update rust crate color-eyre to v0.6.3 ([ac32494](https://github.com/FruitieX/homectl-server/commit/ac324944a5183ca1163181d35ca6b6a181ae2fc2))
* **deps:** update rust crate config to v0.14.0 ([473f589](https://github.com/FruitieX/homectl-server/commit/473f589c38e3264b755fb9b8c0e6712550199f0b))
* **deps:** update rust crate itertools to v0.13.0 ([9ce931a](https://github.com/FruitieX/homectl-server/commit/9ce931a461a82af2542485f4b8456cae92a7a300))
* **deps:** update rust crate jsonptr to v0.4.5 ([3bb9271](https://github.com/FruitieX/homectl-server/commit/3bb927179afc6888277d9e39542ddb9ed7985690))
* **deps:** update rust crate jsonptr to v0.4.6 ([2ca6342](https://github.com/FruitieX/homectl-server/commit/2ca6342bd538692996c78016de3d84024556c47f))
* **deps:** update rust crate jsonptr to v0.4.7 ([f5d8170](https://github.com/FruitieX/homectl-server/commit/f5d81705ad826b5bf60f5dcb88f04dd437e0755b))
* **deps:** update rust crate jsonptr to v0.5.1 ([376cf69](https://github.com/FruitieX/homectl-server/commit/376cf69f202500d955a97eb2389b95d0175366c3))
* **deps:** update rust crate log to v0.4.21 ([8c253cd](https://github.com/FruitieX/homectl-server/commit/8c253cd9db13b58b736a2e9abc59ef91e07ba159))
* **deps:** update rust crate log to v0.4.22 ([9af6b94](https://github.com/FruitieX/homectl-server/commit/9af6b943862ea14384d3155944cc904649fccd19))
* **deps:** update rust crate ordered-float to v4.2.1 ([6553962](https://github.com/FruitieX/homectl-server/commit/6553962b90772fe1d67946b81f72bd4421dfe0c6))
* **deps:** update rust crate palette to v0.7.5 ([3fc62b2](https://github.com/FruitieX/homectl-server/commit/3fc62b28e2d45a1f56b7b7bca56834cf3c2044ba))
* **deps:** update rust crate palette to v0.7.6 ([6acebee](https://github.com/FruitieX/homectl-server/commit/6acebee1c5b2c8e69458cadfc1ee35124c485a88))
* **deps:** update rust crate rumqttc to v0.24.0 ([52d886f](https://github.com/FruitieX/homectl-server/commit/52d886f19ebfa77dc0ca3feccd880b9846f6ee33))
* **deps:** update rust crate serde to v1.0.197 ([40fade7](https://github.com/FruitieX/homectl-server/commit/40fade720d4690af1bed86ce3edd030e679aeeae))
* **deps:** update rust crate serde to v1.0.198 ([a3ff611](https://github.com/FruitieX/homectl-server/commit/a3ff611151028ec6f06c26c1387baef10697d4af))
* **deps:** update rust crate serde to v1.0.199 ([75bcfd4](https://github.com/FruitieX/homectl-server/commit/75bcfd469152b0cbc6806dcb5524d86caf409eac))
* **deps:** update rust crate serde to v1.0.200 ([e9180f9](https://github.com/FruitieX/homectl-server/commit/e9180f9d18969a4dbc966c65f98b913e4b55c29c))
* **deps:** update rust crate serde to v1.0.201 ([6dab1ee](https://github.com/FruitieX/homectl-server/commit/6dab1ee08af4a9d2fb43af9eda63d270c982d630))
* **deps:** update rust crate serde to v1.0.202 ([9c68644](https://github.com/FruitieX/homectl-server/commit/9c686441ba9b9fa0f2594841ef8e5ad4140b8470))
* **deps:** update rust crate serde to v1.0.203 ([d0a37ec](https://github.com/FruitieX/homectl-server/commit/d0a37ec890ef3eec1c4d8dea9fc4bac4ebe7ebdf))
* **deps:** update rust crate serde to v1.0.204 ([fb6be70](https://github.com/FruitieX/homectl-server/commit/fb6be70c3c7df2b9d2683a21158f13d4489b6dcc))
* **deps:** update rust crate serde_json to v1.0.114 ([842f156](https://github.com/FruitieX/homectl-server/commit/842f156d9f4da25ff5fd2f85714eb514ac929265))
* **deps:** update rust crate serde_json to v1.0.115 ([70bf768](https://github.com/FruitieX/homectl-server/commit/70bf768ef105fbc346090f07c6630a35d3ce3d96))
* **deps:** update rust crate serde_json to v1.0.116 ([b7f40f6](https://github.com/FruitieX/homectl-server/commit/b7f40f692c6e26eae7beb14409cd5446ea3275e3))
* **deps:** update rust crate serde_json to v1.0.117 ([ab01f75](https://github.com/FruitieX/homectl-server/commit/ab01f75647adf2371c47ec282fce08a7b9dad73c))
* **deps:** update rust crate serde_json to v1.0.118 ([635d804](https://github.com/FruitieX/homectl-server/commit/635d804f1c2114887c03c51ddf231a5e6bc6e961))
* **deps:** update rust crate serde_json to v1.0.119 ([12bfd02](https://github.com/FruitieX/homectl-server/commit/12bfd02bf2970b48fab47f17772733d3cb7a687f))
* **deps:** update rust crate serde_json to v1.0.120 ([6a543c7](https://github.com/FruitieX/homectl-server/commit/6a543c7eedabdaeec8026c0425a148587924f1a6))
* **deps:** update rust crate serde_json_path to v0.6.6 ([bd10ce5](https://github.com/FruitieX/homectl-server/commit/bd10ce50bba6c71968c89d79cc926016329c5e43))
* **deps:** update rust crate serde_json_path to v0.6.7 ([74f0c29](https://github.com/FruitieX/homectl-server/commit/74f0c29b30b889aa028b98dd9ef12bb40bb47e10))
* **deps:** update rust crate serde_path_to_error to v0.1.16 ([735e147](https://github.com/FruitieX/homectl-server/commit/735e1471f399bdbd8bb824026e55aae8f67bda53))
* **deps:** update rust crate sqlx to v0.7.4 ([5f692b8](https://github.com/FruitieX/homectl-server/commit/5f692b8c81b8929ec3e58bb18e5e2697c8a890e0))
* **deps:** update rust crate sqlx to v0.8.0 ([d7af8fb](https://github.com/FruitieX/homectl-server/commit/d7af8fba83df571b00fe704010d51a1fc621cb23))
* **deps:** update rust crate tokio to v1.37.0 ([8816bda](https://github.com/FruitieX/homectl-server/commit/8816bdabad7a8f1121aeb401cd76133c60b47953))
* **deps:** update rust crate tokio to v1.38.0 ([7b5c620](https://github.com/FruitieX/homectl-server/commit/7b5c620a3036b91e81f863c312d717f8143e1e3c))
* **deps:** update rust crate tokio to v1.38.1 ([af48927](https://github.com/FruitieX/homectl-server/commit/af489274eedfcf8554341db3f7651be4388f9c39))
* **deps:** update rust crate tokio-stream to v0.1.15 ([1ad0680](https://github.com/FruitieX/homectl-server/commit/1ad06801542907e62f5ce6af78600f2f2bcbd972))
* **deps:** update rust crate toml to v0.8.11 ([3114c50](https://github.com/FruitieX/homectl-server/commit/3114c506168e8791db9c35adc338d6a512539ff2))
* **deps:** update rust crate toml to v0.8.12 ([5f8cc7d](https://github.com/FruitieX/homectl-server/commit/5f8cc7d4651a9abf519ab674f69c67c449de99e1))
* **deps:** update rust crate toml to v0.8.13 ([74989ae](https://github.com/FruitieX/homectl-server/commit/74989ae39421195ef162af232df22cd3b8a87a1d))
* **deps:** update rust crate toml to v0.8.14 ([d82ef95](https://github.com/FruitieX/homectl-server/commit/d82ef951834e83de0edfb95c28baeb591930c0c1))
* **deps:** update rust crate toml to v0.8.15 ([501a384](https://github.com/FruitieX/homectl-server/commit/501a3841f695478e5796d2d5a4ba78fd1d79d30c))
* **deps:** update rust crate ts-rs to v8 ([98dbbe3](https://github.com/FruitieX/homectl-server/commit/98dbbe38121e930f88f76d29714aed71fe4d88f4))
* **deps:** update rust crate ts-rs to v8.1.0 ([7170d5f](https://github.com/FruitieX/homectl-server/commit/7170d5f5ad0a894fd9f172fb3ea9b799b1b042d0))
* **deps:** update rust crate ts-rs to v9 ([c94f137](https://github.com/FruitieX/homectl-server/commit/c94f137171baa30f88326258c8cdd75f69894380))
* **deps:** update rust crate ts-rs to v9.0.1 ([88e84da](https://github.com/FruitieX/homectl-server/commit/88e84dab1b9b64f42fb53289718bbe6453981fe0))
* **deps:** update rust crate warp to v0.3.7 ([d024dbe](https://github.com/FruitieX/homectl-server/commit/d024dbeb7bc55c8d9bd461dadfd3c51788e683a5))
* fix not being able to set color after turning on light ([1d0d70c](https://github.com/FruitieX/homectl-server/commit/1d0d70c77f2ec30db4b8e9783980d7dd49919d55))

## [0.9.3](https://github.com/FruitieX/homectl-server/compare/v0.9.2...v0.9.3) (2024-02-19)


### Bug Fixes

* also filter scene expr devices by sd device/group keys ([4dfb3b8](https://github.com/FruitieX/homectl-server/commit/4dfb3b8c9c449aa5ed39cfecd6b6d9938465e50d))

## [0.9.2](https://github.com/FruitieX/homectl-server/compare/v0.9.1...v0.9.2) (2024-02-16)


### Features

* set SetInternalState skip_external_update field as optional ([c3861f0](https://github.com/FruitieX/homectl-server/commit/c3861f0ce35ac8264d8a6ee567414c58fc791615))


### Miscellaneous Chores

* release 0.9.2 ([c6b828e](https://github.com/FruitieX/homectl-server/commit/c6b828ed56603f5fc3d8736b6c69456ae266013a))

## [0.9.1](https://github.com/FruitieX/homectl-server/compare/v0.9.0...v0.9.1) (2024-02-15)


### Features

* use device name in logs ([9576765](https://github.com/FruitieX/homectl-server/commit/95767650fcd9dfea617517ce0e40839703762666))


### Miscellaneous Chores

* release 0.9.1 ([557051f](https://github.com/FruitieX/homectl-server/commit/557051f53a0adc8096a9e79befc39470df94449d))

## [0.9.0](https://github.com/FruitieX/homectl-server/compare/v0.8.0...v0.9.0) (2024-02-15)


### Features

* expected state recomputed only when needed ([da23524](https://github.com/FruitieX/homectl-server/commit/da23524bc35451f6079c4fb2688db9594fbfd345))
* FullReadOnly mode for debugging managed devices ([8b75afb](https://github.com/FruitieX/homectl-server/commit/8b75afb513f8b40bca41c974c82ca0ac5da9ffd2))
* recompute scene device states when scene invalidated ([e961dda](https://github.com/FruitieX/homectl-server/commit/e961dda30b150d74532c99b57d1e228c6787dd13))


### Bug Fixes

* disable state transitions when activating scenes ([db8134f](https://github.com/FruitieX/homectl-server/commit/db8134f5ae03441530e0f8f8e76c4c0a0a9d60a0))

## [0.8.0](https://github.com/FruitieX/homectl-server/compare/v0.7.0...v0.8.0) (2024-02-12)


### Features

* support numeric sensor values ([81f01d1](https://github.com/FruitieX/homectl-server/commit/81f01d1a0036a8a69dc350b0260ef4602f99b470))
* support raw device values ([e37602f](https://github.com/FruitieX/homectl-server/commit/e37602f685da7a542ba8f6a389867611f77cab10))
* wait for devices to be discovered at launch ([de0af33](https://github.com/FruitieX/homectl-server/commit/de0af33bbd0092e28c306e61f5171a9c45f233cf))


### Bug Fixes

* **deps:** update rust crate chrono to v0.4.34 ([9b406b2](https://github.com/FruitieX/homectl-server/commit/9b406b278c09ba45242a3c7c88177fe6de6fa0e0))
* **deps:** update rust crate toml to v0.8.10 ([bca9d3e](https://github.com/FruitieX/homectl-server/commit/bca9d3e74442b8bf6b92ee86da1d37e3b828a618))

## [0.7.0](https://github.com/FruitieX/homectl-server/compare/v0.6.3...v0.7.0) (2024-02-05)


### ⚠ BREAKING CHANGES

* rename Action::DimAction to Action::Dim to make clippy happy

### Features

* avoid computing groups/scene structs on every state update ([997de4e](https://github.com/FruitieX/homectl-server/commit/997de4e6ba116800d1a41f7e6ac97d97be431463))
* cron integration ([3be926f](https://github.com/FruitieX/homectl-server/commit/3be926f4ac30abb39690c72590584a3c07879215))
* evalexpr support in rules and actions ([7453168](https://github.com/FruitieX/homectl-server/commit/74531680169a975fc59977e90d3ae19fcc8cab77))
* partially managed devices ([638e516](https://github.com/FruitieX/homectl-server/commit/638e516fd3b2074d8de1b7af998c1fa09e4adeac))
* ReadOnly managed mode ([48a3934](https://github.com/FruitieX/homectl-server/commit/48a39343a7c902072a1a5976539b6967a5b66192))
* **routines:** expressions in rules and actions ([9632c9f](https://github.com/FruitieX/homectl-server/commit/9632c9f57a42ba86a23746c2e444dc5f07904595))
* scene expressions ([e946d89](https://github.com/FruitieX/homectl-server/commit/e946d89a8a0f95934e284aa8586bc29be3d468e7))
* SetDeviceState action ([fd43901](https://github.com/FruitieX/homectl-server/commit/fd43901ef586a17cb4a0978f82c622a2db553980))
* support forcibly triggering routines ([65c2d77](https://github.com/FruitieX/homectl-server/commit/65c2d775c225b8abf28e8ec8433159d8d3e72e2f))


### Bug Fixes

* always convert color to preferred mode when sending ([c52155c](https://github.com/FruitieX/homectl-server/commit/c52155c6a9788a73c3619cb353a6727ed2acfbff))
* bypass config and use toml crate directly ([a219adb](https://github.com/FruitieX/homectl-server/commit/a219adb34e63b27fbbe918167c016d22aa4dd898))
* **deps:** pin dependencies ([6dd5fcc](https://github.com/FruitieX/homectl-server/commit/6dd5fccf59dfc8affce2fd43774d8175af9c9870))
* **deps:** pin rust crate croner to =2.0.4 ([c4413a1](https://github.com/FruitieX/homectl-server/commit/c4413a12348a2d2711618f8499e4302b67a1f6b4))
* **deps:** pin rust crate jsonptr to =0.4.4 ([bd20093](https://github.com/FruitieX/homectl-server/commit/bd20093018866ad8022beb4290fea578aa3b72f6))
* **deps:** pin rust crate serde-this-or-that to =0.4.2 ([21220ae](https://github.com/FruitieX/homectl-server/commit/21220aed92375bfb5fe08549f57e47fe24ab7ab7))
* **deps:** update rust crate async-trait to v0.1.75 ([18fb828](https://github.com/FruitieX/homectl-server/commit/18fb8282779021daffc46c84bc699dd98d7fd4c4))
* **deps:** update rust crate async-trait to v0.1.76 ([cbb0f3f](https://github.com/FruitieX/homectl-server/commit/cbb0f3fdbb92b0a74aea044dd4399409603a33c3))
* **deps:** update rust crate async-trait to v0.1.77 ([483da8c](https://github.com/FruitieX/homectl-server/commit/483da8c7c56edeb71ffd935a511970f367011484))
* **deps:** update rust crate cached to v0.48.0 ([3b73ce0](https://github.com/FruitieX/homectl-server/commit/3b73ce0eebd88d11271d0e791fd1a898b8888462))
* **deps:** update rust crate cached to v0.48.1 ([504c7c1](https://github.com/FruitieX/homectl-server/commit/504c7c1702ba017f8e566fbe55c0ab6c9cb50ba0))
* **deps:** update rust crate chrono to v0.4.32 ([56ea5b0](https://github.com/FruitieX/homectl-server/commit/56ea5b0fd83680ada64c2898bb0e05dfcd3c9687))
* **deps:** update rust crate chrono to v0.4.33 ([a6fcc3c](https://github.com/FruitieX/homectl-server/commit/a6fcc3c9ed287bb8a72fc3c746e6171c6daa47ec))
* **deps:** update rust crate config to v0.14.0 ([544d9ab](https://github.com/FruitieX/homectl-server/commit/544d9ab72bd7219fe1ce5f0f1ec985a10d8a67e1))
* **deps:** update rust crate eyre to v0.6.10 ([8abf002](https://github.com/FruitieX/homectl-server/commit/8abf002e1140fb5c604c62d0301e6e48bb5fe843))
* **deps:** update rust crate eyre to v0.6.11 ([c1f41c2](https://github.com/FruitieX/homectl-server/commit/c1f41c2041a38d5b69e7b0dab85838991c4f7324))
* **deps:** update rust crate eyre to v0.6.12 ([a54cf04](https://github.com/FruitieX/homectl-server/commit/a54cf04b70ea4caff142833deaf23bdcae4c962b))
* **deps:** update rust crate itertools to v0.12.1 ([a6be06e](https://github.com/FruitieX/homectl-server/commit/a6be06ecbd27c34ce5a23494feddf1ba32a6a038))
* **deps:** update rust crate once_cell to v1.19.0 ([c48f7b5](https://github.com/FruitieX/homectl-server/commit/c48f7b5e192a02e6f1cca176095bfcebc0e2bba9))
* **deps:** update rust crate palette to v0.7.4 ([4d4cf09](https://github.com/FruitieX/homectl-server/commit/4d4cf094c59cb107e9b087054093237a88eb3dc2))
* **deps:** update rust crate serde to v1.0.194 ([4da8780](https://github.com/FruitieX/homectl-server/commit/4da87804d8b7a9f6aa35b8a86b62307a2934a52c))
* **deps:** update rust crate serde to v1.0.195 ([d6baed3](https://github.com/FruitieX/homectl-server/commit/d6baed317a6ee35a0c96bf24e46e9c7ef395a3f7))
* **deps:** update rust crate serde to v1.0.196 ([c25c6bf](https://github.com/FruitieX/homectl-server/commit/c25c6bf8e48704ebfa16e00135db6b619307f94a))
* **deps:** update rust crate serde_json to v1.0.109 ([05a0e92](https://github.com/FruitieX/homectl-server/commit/05a0e92e6168563f54e66305f30cbca2d43872fe))
* **deps:** update rust crate serde_json to v1.0.110 ([b7f92ab](https://github.com/FruitieX/homectl-server/commit/b7f92aba4320b9f4f0b352d36a7d26ec8afdc68d))
* **deps:** update rust crate serde_json to v1.0.111 ([07fd30d](https://github.com/FruitieX/homectl-server/commit/07fd30d3668b83a8744a0247233914578cb816ab))
* **deps:** update rust crate serde_json to v1.0.112 ([aa72849](https://github.com/FruitieX/homectl-server/commit/aa7284937c7f34936c7de4e00dbd4126e00c1918))
* **deps:** update rust crate serde_json to v1.0.113 ([4dae8b3](https://github.com/FruitieX/homectl-server/commit/4dae8b3e1c29efe6e987c428f4f66d1c631ab589))
* **deps:** update rust crate serde_path_to_error to v0.1.15 ([8ecd7f4](https://github.com/FruitieX/homectl-server/commit/8ecd7f493c62a72806c34b22ecc59ecca0f5a5ec))
* **deps:** update rust crate tokio to v1.35.0 ([3c3962e](https://github.com/FruitieX/homectl-server/commit/3c3962e19e43335cdca355fe0b74fb0530b10a2e))
* **deps:** update rust crate tokio to v1.35.1 ([1633046](https://github.com/FruitieX/homectl-server/commit/16330462335ac83ee601fc957fd5d65c0f729212))
* **deps:** update rust crate tokio to v1.36.0 ([93c4479](https://github.com/FruitieX/homectl-server/commit/93c44790de31c6ffcd27c105104f4a6871cd4bdd))
* **deps:** update rust crate toml to v0.8.9 ([060f1e5](https://github.com/FruitieX/homectl-server/commit/060f1e5d288670ea9b0d257769293c41b08ec0a1))
* **deps:** update rust crate ts-rs to v7.1.0 ([40d1edf](https://github.com/FruitieX/homectl-server/commit/40d1edf6fefd3af01c6da92fa95b1cbe7dd93a7b))
* **deps:** update rust crate ts-rs to v7.1.1 ([72d0bb9](https://github.com/FruitieX/homectl-server/commit/72d0bb9feaa2474734955b93f31ddea83b531f9f))
* **deps:** update rust-futures monorepo to v0.3.30 ([2b396c9](https://github.com/FruitieX/homectl-server/commit/2b396c9cc1f959f0d802e3ab5a6bc59bca02f225))
* don't spawn new task for each message ([e6a6e89](https://github.com/FruitieX/homectl-server/commit/e6a6e897746a01ffc050b36373cd9523d55c0600))
* drop neato and wake_on_lan integrations ([161480d](https://github.com/FruitieX/homectl-server/commit/161480d5ad2bae09f7a07e0ae8c608c191c57b41))
* improve method of detecting written expr vars ([45125e8](https://github.com/FruitieX/homectl-server/commit/45125e8764d0cfe6cc05ed8649b138586274a289))
* make more use of cached flattened groups config ([b18d32c](https://github.com/FruitieX/homectl-server/commit/b18d32ceca258333acff13ee84222e771a6c8e76))
* websockets don't hold onto state lock forever ([360d804](https://github.com/FruitieX/homectl-server/commit/360d804bd144a585935f74f308c54ce9eef07a1a))


### Miscellaneous Chores

* release 0.7.0 ([ba4fca7](https://github.com/FruitieX/homectl-server/commit/ba4fca7478b02eb7014f2a93cc6303ae96b27430))
* rename Action::DimAction to Action::Dim to make clippy happy ([3851b92](https://github.com/FruitieX/homectl-server/commit/3851b927382c067c050b02cd3d627c8d5dc4dbf1))

## [0.6.3](https://github.com/FruitieX/homectl-server/compare/v0.6.2...v0.6.3) (2023-11-25)


### Bug Fixes

* always broadcast state updates to ws ([7bd6d70](https://github.com/FruitieX/homectl-server/commit/7bd6d70d622eae0139bde14d35878e771aae16e4))

## [0.6.2](https://github.com/FruitieX/homectl-server/compare/v0.6.1...v0.6.2) (2023-11-25)


### Bug Fixes

* core takes care of correct unmanaged msg type ([27e36d3](https://github.com/FruitieX/homectl-server/commit/27e36d365086d8a79d9cf97611d425543fc8a6c3))
* move the managed flag inside DeviceData::Controllable ([6bd7740](https://github.com/FruitieX/homectl-server/commit/6bd77409f793e9632313775a1f4b6d949c78fb47))

## [0.6.1](https://github.com/FruitieX/homectl-server/compare/v0.6.0...v0.6.1) (2023-11-25)


### Bug Fixes

* unmanaged device updates don't emit SendDeviceState ([2d2223e](https://github.com/FruitieX/homectl-server/commit/2d2223e4c384ff365b8159bf618fc07c09790bac))

## [0.6.0](https://github.com/FruitieX/homectl-server/compare/v0.5.1...v0.6.0) (2023-11-25)


### Features

* **mqtt:** unmanaged mqtt devices ([d2352e0](https://github.com/FruitieX/homectl-server/commit/d2352e043190face0357cb6d93e58f93057c65ff))


### Bug Fixes

* **deps:** update rust crate config to v0.13.4 ([3b9588c](https://github.com/FruitieX/homectl-server/commit/3b9588cfb74034605f66f82147c3e6ad543a69fc))
* **deps:** update rust crate eyre to v0.6.9 ([315456f](https://github.com/FruitieX/homectl-server/commit/315456f9bbd6920e9fda19bad7bf46bfb7476e64))
* **deps:** update rust crate itertools to v0.12.0 ([0865780](https://github.com/FruitieX/homectl-server/commit/086578010f7261edf6271c246b0b0f5e143d2bd1))
* **deps:** update rust crate serde to 1.0.190 ([8f81352](https://github.com/FruitieX/homectl-server/commit/8f81352b9298fe598857b244389959a1b0e82ddc))
* **deps:** update rust crate serde to v1.0.192 ([548d501](https://github.com/FruitieX/homectl-server/commit/548d5019dc658089ded210f4ecbd5ed4c659e3f5))
* **deps:** update rust crate serde to v1.0.193 ([504790e](https://github.com/FruitieX/homectl-server/commit/504790e8ce2093aad89e3670d77a64d9a11f237c))
* **deps:** update rust crate serde_json to v1.0.108 ([fb18f91](https://github.com/FruitieX/homectl-server/commit/fb18f914aad62c46544b2874846a7f5bb3176b7d))
* **deps:** update rust crate sqlx to v0.7.3 ([5db2ed9](https://github.com/FruitieX/homectl-server/commit/5db2ed9d87a3393c84a96fe08b52896ca807cb46))
* **deps:** update rust crate tokio to v1.34.0 ([45ad1de](https://github.com/FruitieX/homectl-server/commit/45ad1de6d7aa94cccb71c7b58af1c8506031298a))
* **deps:** update rust crate toml to 0.8.4 ([1115970](https://github.com/FruitieX/homectl-server/commit/1115970db6aa7eed2d1dec4c9a7ead797f81b5f3))
* **deps:** update rust crate toml to v0.8.5 ([8ce7c73](https://github.com/FruitieX/homectl-server/commit/8ce7c731106ace1ba1e579de87a647bf4c1c4c1c))
* **deps:** update rust crate toml to v0.8.6 ([71b03f1](https://github.com/FruitieX/homectl-server/commit/71b03f13b04367c342651ae231c949b25385ad86))
* **deps:** update rust crate toml to v0.8.8 ([9dbc604](https://github.com/FruitieX/homectl-server/commit/9dbc604e57294be4ff6bff4bd8a7e920ed755e7c))
* **deps:** update rust-futures monorepo to v0.3.29 ([2e87670](https://github.com/FruitieX/homectl-server/commit/2e876700db508decc08cbf221e8e4b20f8026355))
* perform db updates in separate task ([ff04147](https://github.com/FruitieX/homectl-server/commit/ff041479621d3263dc0acb628be1042c1904daa7))
* remove redundant device db update call ([23e985e](https://github.com/FruitieX/homectl-server/commit/23e985eeb9a1a461ad44a59e99ede7973e4b8442))
* warn if scenes table is busted ([6a8ed45](https://github.com/FruitieX/homectl-server/commit/6a8ed458a8789a1f8fd3e60c50579634c098f28b))

## [0.5.1](https://github.com/FruitieX/homectl-server/compare/v0.5.0...v0.5.1) (2023-10-20)


### Miscellaneous Chores

* release 0.5.1 ([1b97e07](https://github.com/FruitieX/homectl-server/commit/1b97e070d2193591a9c352d7b149bf10edadfe64))

## [0.5.0](https://github.com/FruitieX/homectl-server/compare/v0.4.5...v0.5.0) (2023-10-20)


### Features

* adds dim/brighten action for lights ([89382fa](https://github.com/FruitieX/homectl-server/commit/89382fa83371c3c4c8de9135109a12d9f28e9217))


### Bug Fixes

* **deps:** update all non-major dependencies ([6318096](https://github.com/FruitieX/homectl-server/commit/63180960410700a786631c4333d8e54c4d0911db))
* **deps:** update rust crate async-trait to 0.1.69 ([74b3985](https://github.com/FruitieX/homectl-server/commit/74b3985803e13fc600d0abdf5ab50ccd8dd53b02))
* **deps:** update rust crate json_value_merge to v2 ([7940e43](https://github.com/FruitieX/homectl-server/commit/7940e437fa8b7374b1292ae093ff496964a8f431))
* **deps:** update rust crate serde to 1.0.166 ([b38995b](https://github.com/FruitieX/homectl-server/commit/b38995bed70fc7d332707f8d0d61445f691ee8cb))
* **deps:** update rust crate ts-rs to v7 ([eedf5b8](https://github.com/FruitieX/homectl-server/commit/eedf5b84593622214ff57dacb642a1476970041d))
* MQTT client re-subscribes on reconnect ([7f1ef3d](https://github.com/FruitieX/homectl-server/commit/7f1ef3da3dfc72240fef37ebfb4a6309d7d1a6c6))

## [0.4.5](https://github.com/FruitieX/homectl-server/compare/v0.4.4...v0.4.5) (2023-06-29)


### Features

* don't convert Ct colors in API responses ([9d7142c](https://github.com/FruitieX/homectl-server/commit/9d7142c37e7c28b66235670da0957e312dc46274))
* improved error reporting with color_eyre ([ecb2163](https://github.com/FruitieX/homectl-server/commit/ecb21637b5b1fae06548abd7e716a408cd815001))


### Miscellaneous Chores

* release 0.4.5 ([75acffd](https://github.com/FruitieX/homectl-server/commit/75acffd3b9f5339a66858eb6db695bb512cc79f4))

## [0.4.4](https://github.com/FruitieX/homectl-server/compare/v0.4.3...v0.4.4) (2023-06-26)


### Features

* perform all logging via pretty_env_logger ([87a2290](https://github.com/FruitieX/homectl-server/commit/87a2290242136b5e50e648504f915f0e08453757))


### Bug Fixes

* attempt reconnecting to mqtt after failure ([c731799](https://github.com/FruitieX/homectl-server/commit/c731799df148d19c66b342312e90b6da567d0a91))
* **deps:** update rust crate itertools to 0.11.0 ([ce60f83](https://github.com/FruitieX/homectl-server/commit/ce60f8367aafa6dc1872dd14486ec4b1cee88c12))
* **deps:** update rust crate toml to 0.7.5 ([af7687f](https://github.com/FruitieX/homectl-server/commit/af7687fced4941102f5b6a0d1bc1ad33946f67bd))

## [0.4.3](https://github.com/FruitieX/homectl-server/compare/v0.4.2...v0.4.3) (2023-06-17)


### Bug Fixes

* don't set default brightness when power is false ([bfba94d](https://github.com/FruitieX/homectl-server/commit/bfba94d5de5b0970cc57cde9dd43758f73f116ff))

## [0.4.2](https://github.com/FruitieX/homectl-server/compare/v0.4.1...v0.4.2) (2023-06-16)


### Bug Fixes

* convert device state to Hs mode in api responses ([875e950](https://github.com/FruitieX/homectl-server/commit/875e950749284b42259573678ee116645b3f1def))

## [0.4.1](https://github.com/FruitieX/homectl-server/compare/v0.4.0...v0.4.1) (2023-06-16)


### Features

* support specifying color mode in get devices endpoint ([9397fea](https://github.com/FruitieX/homectl-server/commit/9397feaa89331f6019fb96bde2c1b2135fbf5692))


### Miscellaneous Chores

* release 0.4.1 ([907d8b0](https://github.com/FruitieX/homectl-server/commit/907d8b0615c663a68fbd73eeb780fff907214390))

## [0.4.0](https://github.com/FruitieX/homectl-server/compare/v0.2.0...v0.4.0) (2023-06-16)


### ⚠ BREAKING CHANGES

* removed hue, lifx, ping integrations in favor of mqtt integrations. Migrate to e.g. [hue-mqtt](https://github.com/FruitieX/hue-mqtt), [lifx-mqtt](https://github.com/FruitieX/lifx-mqtt) using the `mqtt` integration instead.
* the shape of device state has changed in API endpoints, config files, db rows. HSV colors are now represented as `color = { h = 42, s = 0.5 }`. Value is ignored, use brightness on the device instead.

### Features

* compare device color in preferred color format ([46f6c05](https://github.com/FruitieX/homectl-server/commit/46f6c050170504ee357530df9804f7dcc36e3eec))
* **dummy:** support all device types ([58d445e](https://github.com/FruitieX/homectl-server/commit/58d445e023894a8422666458331f5106424eef2b))
* **mqtt:** support publishing arbitrary messages ([611dbd2](https://github.com/FruitieX/homectl-server/commit/611dbd2dbafebf01b4da354c2130b2a72b77720d))
* **wol:** allow supplying broadcast SocketAddr ([7742221](https://github.com/FruitieX/homectl-server/commit/77422216739b12dda33d461d665758737023521c))


### Bug Fixes

* **deps:** update all non-major dependencies ([88dba85](https://github.com/FruitieX/homectl-server/commit/88dba856d5987b48e10734064a2e7e8422487be6))
* **deps:** update rust crate chrono to 0.4.26 ([9cb1006](https://github.com/FruitieX/homectl-server/commit/9cb1006205756327aa01c7e29c2fd076f4a88f41))
* **deps:** update rust crate log to 0.4.19 ([246487d](https://github.com/FruitieX/homectl-server/commit/246487d6633c48e7c1fc6cff42defab9fae6d773))
* **deps:** update rust crate once_cell to 1.18.0 ([f6fcd48](https://github.com/FruitieX/homectl-server/commit/f6fcd487f3e6c525a45dc535c29245188428f132))
* **deps:** update rust crate palette to 0.7.2 ([23499c7](https://github.com/FruitieX/homectl-server/commit/23499c7030b10b52a69f50ea84f00e3aa6620776))
* **deps:** update rust crate rumqttc to 0.22.0 ([3c38c8b](https://github.com/FruitieX/homectl-server/commit/3c38c8b5bc96a66a8f5edf85d4965c6b745906e7))
* **deps:** update rust crate serde to 1.0.164 ([2f5369b](https://github.com/FruitieX/homectl-server/commit/2f5369bee3832daacacecb00a58fb823cfc9ace1))
* **deps:** update rust crate sha2 to 0.10.7 ([c5f26b4](https://github.com/FruitieX/homectl-server/commit/c5f26b4566c302963ae3d4296d2f77694f8bd283))
* **deps:** update rust crate toml to 0.7.4 ([dd85f9b](https://github.com/FruitieX/homectl-server/commit/dd85f9b357d0694f378999ece785d5e487acbe8d))
* don't send device update upon restore from db ([ff265f6](https://github.com/FruitieX/homectl-server/commit/ff265f68c2c236d5407cae2428c20f23363c2035))
* improve formatting of printed state mismatch messages ([6749d45](https://github.com/FruitieX/homectl-server/commit/6749d45e55b222677d60a99ca4f8753ff83e1c74))
* incorrect put_device endpoint path ([65c7a45](https://github.com/FruitieX/homectl-server/commit/65c7a45762b59a2c1799463d480e34fcc67985f0))
* missing scene brightness bug ([6749d45](https://github.com/FruitieX/homectl-server/commit/6749d45e55b222677d60a99ca4f8753ff83e1c74))
* **neato:** check time of day even with force flag ([ed8c5a4](https://github.com/FruitieX/homectl-server/commit/ed8c5a4ec56d6307a68497ffb4b171669ec118cf))
* remove unused variable ([07b4b33](https://github.com/FruitieX/homectl-server/commit/07b4b33f89eb5b8819e6e63d5f561681c4fccad5))
* set default working directory in Dockerfile ([4ddd5ed](https://github.com/FruitieX/homectl-server/commit/4ddd5edb3bd59b63f77d67f7c2bf7db93f0ecce0))


### Code Refactoring

* remove outdated code ([d03cb8f](https://github.com/FruitieX/homectl-server/commit/d03cb8f5319578b10d6b4ba543e319bedfc49e92))
* simplify device structs ([d03cb8f](https://github.com/FruitieX/homectl-server/commit/d03cb8f5319578b10d6b4ba543e319bedfc49e92))


### Miscellaneous Chores

* release 0.4.0 ([6e7a0ee](https://github.com/FruitieX/homectl-server/commit/6e7a0ee4c13e2cb3fbf7b137548cdf7a9249d6d0))

## [0.3.0](https://github.com/FruitieX/homectl-server/compare/v0.2.0...v0.3.0) (2023-05-31)


### Features

* **dummy:** support all device types ([58d445e](https://github.com/FruitieX/homectl-server/commit/58d445e023894a8422666458331f5106424eef2b))
* **mqtt:** support publishing arbitrary messages ([611dbd2](https://github.com/FruitieX/homectl-server/commit/611dbd2dbafebf01b4da354c2130b2a72b77720d))
* **wol:** allow supplying broadcast SocketAddr ([7742221](https://github.com/FruitieX/homectl-server/commit/77422216739b12dda33d461d665758737023521c))


### Bug Fixes

* **deps:** update all non-major dependencies ([88dba85](https://github.com/FruitieX/homectl-server/commit/88dba856d5987b48e10734064a2e7e8422487be6))
* **deps:** update rust crate chrono to 0.4.26 ([9cb1006](https://github.com/FruitieX/homectl-server/commit/9cb1006205756327aa01c7e29c2fd076f4a88f41))
* **deps:** update rust crate palette to 0.7.2 ([23499c7](https://github.com/FruitieX/homectl-server/commit/23499c7030b10b52a69f50ea84f00e3aa6620776))
* **deps:** update rust crate toml to 0.7.4 ([dd85f9b](https://github.com/FruitieX/homectl-server/commit/dd85f9b357d0694f378999ece785d5e487acbe8d))
* don't send device update upon restore from db ([ff265f6](https://github.com/FruitieX/homectl-server/commit/ff265f68c2c236d5407cae2428c20f23363c2035))
* incorrect put_device endpoint path ([65c7a45](https://github.com/FruitieX/homectl-server/commit/65c7a45762b59a2c1799463d480e34fcc67985f0))
* **neato:** check time of day even with force flag ([ed8c5a4](https://github.com/FruitieX/homectl-server/commit/ed8c5a4ec56d6307a68497ffb4b171669ec118cf))
* remove unused variable ([07b4b33](https://github.com/FruitieX/homectl-server/commit/07b4b33f89eb5b8819e6e63d5f561681c4fccad5))

## 0.2.0 (2023-05-17)


### Miscellaneous Chores

* release 0.2.0 ([4bb7c5d](https://github.com/FruitieX/homectl-server/commit/4bb7c5d0e5a64e5265aff75802271cee92317b23))
