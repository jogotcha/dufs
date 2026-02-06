# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0](https://github.com/jogotcha/dufs/compare/dufs-v0.45.0...dufs-v1.0.0) (2026-02-06)


### ⚠ BREAKING CHANGES

* new auth
* explicitly allow download folder as zip file
* rename duf to dufs
* `--cors` rename to `--enable-cors`
* `--auth` is changed, `--no-auth-access` is removed

### Features

* add --assets options to override assets ([#134](https://github.com/jogotcha/dufs/issues/134)) ([a74e40a](https://github.com/jogotcha/dufs/commit/a74e40aee59f6eafe1080308fd1ad5af8da1ec65))
* add `--compress` option ([#319](https://github.com/jogotcha/dufs/issues/319)) ([3873f47](https://github.com/jogotcha/dufs/commit/3873f4794af3f301bf371bbc6b8ad9aa003e6d5e))
* add api to get the hash of a file ([#375](https://github.com/jogotcha/dufs/issues/375)) ([9353b2e](https://github.com/jogotcha/dufs/commit/9353b2e75908a4c00cfdb35df34c88e7c0de472e))
* add basic auth and readonly mode ([78e7daf](https://github.com/jogotcha/dufs/commit/78e7daf7cc4abfff156b13f88da9cbd7827370c5))
* add basic dark theme ([#29](https://github.com/jogotcha/dufs/issues/29)) ([46ebe97](https://github.com/jogotcha/dufs/commit/46ebe978ae44a27e38a1acd6b41a9ae5e037b484))
* add cache-control:no-cache while sending file and index ([#528](https://github.com/jogotcha/dufs/issues/528)) ([669c4f8](https://github.com/jogotcha/dufs/commit/669c4f88114ae2c80746af645b29c03c3f7adb7f))
* add completions ([#97](https://github.com/jogotcha/dufs/issues/97)) ([76e967f](https://github.com/jogotcha/dufs/commit/76e967fa59a44417581914a62f72039fd20bd157))
* add cors ([10aabcb](https://github.com/jogotcha/dufs/commit/10aabcb2f2167c550417a74ff9996e7d4d591555))
* add empty state placeholder to page([#30](https://github.com/jogotcha/dufs/issues/30)) ([0e236b6](https://github.com/jogotcha/dufs/commit/0e236b61f6a389381b1cfe42012b361557822126))
* add favicon ([#27](https://github.com/jogotcha/dufs/issues/27)) ([261c8b6](https://github.com/jogotcha/dufs/commit/261c8b6ee57458c2ebb3f266ee4f7eb59482394b)), closes [#16](https://github.com/jogotcha/dufs/issues/16)
* add filetype symbols ([3ea814c](https://github.com/jogotcha/dufs/commit/3ea814c8f2b7c2cef1627d51df9c22958710374f))
* add filetype symbols to listings ([d47a748](https://github.com/jogotcha/dufs/commit/d47a748f796d9de4122fcb297453e25c2b3e4090))
* add log-file option ([#383](https://github.com/jogotcha/dufs/issues/383)) ([6b6d69a](https://github.com/jogotcha/dufs/commit/6b6d69a8ef41f6208169b4c637f9bea5217a1faa))
* add logger ([63e6906](https://github.com/jogotcha/dufs/commit/63e69063939f821b41069781673230684d1678e9))
* add mime and cache headers to response ([d8d5aae](https://github.com/jogotcha/dufs/commit/d8d5aae898e94b2017ad633e81ff357fc83be85d))
* add no-auth-read options ([d9547ad](https://github.com/jogotcha/dufs/commit/d9547ad00b26dbeb2e1e31cffc58030593aee2a1))
* add option --allow-archive ([#152](https://github.com/jogotcha/dufs/issues/152)) ([8d9705c](https://github.com/jogotcha/dufs/commit/8d9705caa40b6ed54e8a3350694e67a50c359009))
* add option --allow-hash to allow/disallow file hashing ([#657](https://github.com/jogotcha/dufs/issues/657)) ([2b2c7bd](https://github.com/jogotcha/dufs/commit/2b2c7bd5f718b0516d3a229e91ede5c0d2e32bb3))
* add option --allow-search ([#62](https://github.com/jogotcha/dufs/issues/62)) ([4058a2d](https://github.com/jogotcha/dufs/commit/4058a2db72e05cb58e678c992bf2e24b2521c649))
* add option --render-try-index ([#47](https://github.com/jogotcha/dufs/issues/47)) ([5b11bb7](https://github.com/jogotcha/dufs/commit/5b11bb75dde7fc7ee25056f6b5c9f3e071f3b980)), closes [#46](https://github.com/jogotcha/dufs/issues/46)
* add release-please workflow and configuration files [skip-ci] ([6018acc](https://github.com/jogotcha/dufs/commit/6018acc51750e63efddb5a9022bb846a4e2f2dbf))
* add slash to end of dir href ([68d238d](https://github.com/jogotcha/dufs/commit/68d238d34db7c162214ae5598ad2164fc8ec3671))
* add some headers to res ([b883811](https://github.com/jogotcha/dufs/commit/b88381167bd0deafa86224f0edb30d7b51d11a2e))
* add timestamp metadata to generated zip file ([#204](https://github.com/jogotcha/dufs/issues/204)) ([652f836](https://github.com/jogotcha/dufs/commit/652f836c2354408052d0a839eac9fbb4c141fe09))
* add webdav proppatch handler ([#18](https://github.com/jogotcha/dufs/issues/18)) ([5578ee9](https://github.com/jogotcha/dufs/commit/5578ee91903c7624a3f390e35c754cfa47b06fae))
* added basic auth ([#60](https://github.com/jogotcha/dufs/issues/60)) ([deb6365](https://github.com/jogotcha/dufs/commit/deb6365a2830983f82be4b299a4175887928afd9))
* adjust digest auth timeout to 1day ([#110](https://github.com/jogotcha/dufs/issues/110)) ([9cfd66d](https://github.com/jogotcha/dufs/commit/9cfd66dab96a1dd04a98c3b6d28c82c5a027c722))
* allow search with --render-try-index ([#88](https://github.com/jogotcha/dufs/issues/88)) ([2ffdcdf](https://github.com/jogotcha/dufs/commit/2ffdcdf10666585e7864027e52ef73505f1ad176))
* API to search and list directories ([#177](https://github.com/jogotcha/dufs/issues/177)) ([7d6d7d4](https://github.com/jogotcha/dufs/commit/7d6d7d49ca0b31226a72b8f84501205449b02388)), closes [#166](https://github.com/jogotcha/dufs/issues/166)
* auth supports forbidden permissions ([#329](https://github.com/jogotcha/dufs/issues/329)) ([af347f9](https://github.com/jogotcha/dufs/commit/af347f9cf0325a0f6f4f72c79172a39c5a26cdf6))
* Automatically create dir while uploading ([c8a25b5](https://github.com/jogotcha/dufs/commit/c8a25b54abdcc0991cf4d93993c4d8567ee5820f))
* aware RUST_LOG ([3673a64](https://github.com/jogotcha/dufs/commit/3673a64ec777ba761cc6ef24cfb0766bef50081e))
* base64 index-data to avoid misencoding ([#421](https://github.com/jogotcha/dufs/issues/421)) ([ca5c3d7](https://github.com/jogotcha/dufs/commit/ca5c3d7c541f7aa89c42db312d12e634cdc41460))
* change auth logic/options ([c50f979](https://github.com/jogotcha/dufs/commit/c50f97925c7afc0b5a936d438a5505d166fb5c76))
* check permission on move/copy destination ([#93](https://github.com/jogotcha/dufs/issues/93)) ([604cbb7](https://github.com/jogotcha/dufs/commit/604cbb74122bba31cee84a2278279a28d2d6979a))
* cli add allow-symlink option ([a9294f6](https://github.com/jogotcha/dufs/commit/a9294f602cdf691b294e11e02c5ac5ce84d8c60d))
* delete confirm ([be3ae2f](https://github.com/jogotcha/dufs/commit/be3ae2fe0077f602e643b88c09237b8c5372879f)), closes [#1](https://github.com/jogotcha/dufs/issues/1)
* deprecate `--auth-method`,  as both options are available ([#279](https://github.com/jogotcha/dufs/issues/279)) ([70300b1](https://github.com/jogotcha/dufs/commit/70300b133ccb0ee652511ccffe4d4ac5284e614e))
* deprecate the use of `|` to separate auth rules ([#298](https://github.com/jogotcha/dufs/issues/298)) ([7584fe3](https://github.com/jogotcha/dufs/commit/7584fe3d08b2c0d15cd456e133f21c768d577c84))
* display upload speed and time left ([#34](https://github.com/jogotcha/dufs/issues/34)) ([b3890ea](https://github.com/jogotcha/dufs/commit/b3890ea094ce5578bdd2b6688cd7bba3c0f37750))
* distinct upload and delete operation ([3032052](https://github.com/jogotcha/dufs/commit/3032052923063f4f2b4735b8bf80fdd7d67c000e))
* download folder as zip file ([ae11a39](https://github.com/jogotcha/dufs/commit/ae11a39804a3db280a087906ce42084baef22a3e))
* drag and drop uploads, upload folder ([19d7b36](https://github.com/jogotcha/dufs/commit/19d7b36462e8dd0da587440515c2e7632ad0f277)), closes [#3](https://github.com/jogotcha/dufs/issues/3)
* empty search `?q=` list all paths ([#311](https://github.com/jogotcha/dufs/issues/311)) ([5c85025](https://github.com/jogotcha/dufs/commit/5c850256f471d567e22eaae01dbaf565d589e5cd))
* guess plain text encoding then set content-type charset ([#186](https://github.com/jogotcha/dufs/issues/186)) ([45f4f5f](https://github.com/jogotcha/dufs/commit/45f4f5fc58eb09a9a01c546810fd3230aa6a9c51))
* hiding only directories instead of files ([#175](https://github.com/jogotcha/dufs/issues/175)) ([b7c5119](https://github.com/jogotcha/dufs/commit/b7c5119c2e1985e21a625c20710c318c8b1831f4))
* higher perm auth path shadows lower one ([#521](https://github.com/jogotcha/dufs/issues/521)) ([e576ddc](https://github.com/jogotcha/dufs/commit/e576ddcbea57c03e3d8ae0168416d688411ded4c))
* implement head method ([#33](https://github.com/jogotcha/dufs/issues/33)) ([99f0de6](https://github.com/jogotcha/dufs/commit/99f0de6ca066de9b43fae2a4ca3c66665a982db5))
* implement more webdav methods ([#13](https://github.com/jogotcha/dufs/issues/13)) ([05155aa](https://github.com/jogotcha/dufs/commit/05155aa532ba1c8ef3ecec290feb743fe3cce4d9))
* implements remaining http cache conditionalss ([#407](https://github.com/jogotcha/dufs/issues/407)) ([632f7a4](https://github.com/jogotcha/dufs/commit/632f7a41bf7a362747ee22ce50f70ec69d12e766))
* improve hidden to support glob ([#108](https://github.com/jogotcha/dufs/issues/108)) ([b791549](https://github.com/jogotcha/dufs/commit/b791549ec777d506265cd24d547d286d067e50c9))
* improve ui ([0a3d9c3](https://github.com/jogotcha/dufs/commit/0a3d9c391f7dbc5ae9936d1f70a15669195cc223))
* limit sub directory item counting ([#556](https://github.com/jogotcha/dufs/issues/556)) ([d0453b7](https://github.com/jogotcha/dufs/commit/d0453b7591fa3d5d41f202661e16ba19dc7a50f0))
* limit the number of concurrent uploads ([#98](https://github.com/jogotcha/dufs/issues/98)) ([05dbcfb](https://github.com/jogotcha/dufs/commit/05dbcfb2df3fb4cbba56dfdc512a81c9525ce162))
* list all ifaces when listening 0.0.0.0 ([9797871](https://github.com/jogotcha/dufs/commit/97978719b366ee2c85a0eef9534a92f9225db370))
* listen 0.0.0.0 by default ([9dda55b](https://github.com/jogotcha/dufs/commit/9dda55b7c8068e0935a61cb7adae80e3ebc5f8d8))
* listen both ipv4 and ipv6 by default ([#40](https://github.com/jogotcha/dufs/issues/40)) ([12aafa0](https://github.com/jogotcha/dufs/commit/12aafa00d82ff97f65169e641b73082d5f0f8ef3))
* log decoded uri ([#615](https://github.com/jogotcha/dufs/issues/615)) ([7f82698](https://github.com/jogotcha/dufs/commit/7f8269881df3113d5310f8f47d1f79eb6bac13ea))
* make --path-prefix works on serving single file ([#102](https://github.com/jogotcha/dufs/issues/102)) ([4e823e8](https://github.com/jogotcha/dufs/commit/4e823e8bba1480c804f09e4ac88f47c0966b1f25))
* make dir urls inherit `?noscript` params ([#614](https://github.com/jogotcha/dufs/issues/614)) ([b2f244a](https://github.com/jogotcha/dufs/commit/b2f244a4cfeb492b38ad9b92692e230e04540ea0))
* more flexible config values ([#299](https://github.com/jogotcha/dufs/issues/299)) ([6ff8b29](https://github.com/jogotcha/dufs/commit/6ff8b29b691512e7d6964775e6e0a2385bfa395d))
* new auth ([#218](https://github.com/jogotcha/dufs/issues/218)) ([f8ea416](https://github.com/jogotcha/dufs/commit/f8ea41638f16763b4d7759933fe464740011ae0e))
* options method return status 200 ([dd8b21f](https://github.com/jogotcha/dufs/commit/dd8b21f3a6d056b86000868c8145876e4a271104))
* password can contain `:` `@` `|` ([#297](https://github.com/jogotcha/dufs/issues/297)) ([653cd16](https://github.com/jogotcha/dufs/commit/653cd167d09d0b9a3042d195e71aee1f13f0a45f))
* path level access control ([#52](https://github.com/jogotcha/dufs/issues/52)) ([9c2e9d1](https://github.com/jogotcha/dufs/commit/9c2e9d1503ec1eb18cd1195d795a570f82b3117a))
* provide healthcheck API ([#474](https://github.com/jogotcha/dufs/issues/474)) ([d445b78](https://github.com/jogotcha/dufs/commit/d445b78f96d95bc12f124e82751ba950dedf6281))
* reactive webpage ([#51](https://github.com/jogotcha/dufs/issues/51)) ([b1b0fdd](https://github.com/jogotcha/dufs/commit/b1b0fdd4dbc7df29a91c6f9dc955cd9b0ee3567d))
* remove parent path ([844c626](https://github.com/jogotcha/dufs/commit/844c6265c019f53f4ca65b5d8c7d55b9a306e024))
* remove unzip uploaded feature ([#11](https://github.com/jogotcha/dufs/issues/11)) ([0616602](https://github.com/jogotcha/dufs/commit/0616602659969354d6d125b972f93e7e56c21ed3))
* rename to dufs ([#59](https://github.com/jogotcha/dufs/issues/59)) ([a67da8b](https://github.com/jogotcha/dufs/commit/a67da8bdd31ef021e32074c1e0ac391bc5d1022b)), closes [#50](https://github.com/jogotcha/dufs/issues/50)
* replace --static option to --no-edit ([06ce7b0](https://github.com/jogotcha/dufs/commit/06ce7b01758fcca5bcb789aaaadb6f945efc025d))
* revert supporting for forbidden permission ([#352](https://github.com/jogotcha/dufs/issues/352)) ([95eb648](https://github.com/jogotcha/dufs/commit/95eb648411d4afe71460d86f3f64cdc7b2207281))
* serve single file ([#54](https://github.com/jogotcha/dufs/issues/54)) ([c3ac2a2](https://github.com/jogotcha/dufs/commit/c3ac2a21c95d8f687e8af8821fe4385d1810bd67)), closes [#53](https://github.com/jogotcha/dufs/issues/53)
* show precise file size with decimal ([#210](https://github.com/jogotcha/dufs/issues/210)) ([8a1e767](https://github.com/jogotcha/dufs/commit/8a1e7674df37ab18924dd189d8b450dfb5357ee2))
* sort by type first, then sort by name/mtime/size ([#241](https://github.com/jogotcha/dufs/issues/241)) ([d9706d7](https://github.com/jogotcha/dufs/commit/d9706d75ef003db3bef23c960e75317c8698ffec))
* support  downloading via token auth ([#603](https://github.com/jogotcha/dufs/issues/603)) ([9c9fca7](https://github.com/jogotcha/dufs/commit/9c9fca75d357da2e8e91f2ebbf406b88792c307a))
* support binding abstract unix socket ([#468](https://github.com/jogotcha/dufs/issues/468)) ([881a67e](https://github.com/jogotcha/dufs/commit/881a67e1a462b6ecc9f5099a7bfa7a32b37573c5))
* support config file with `--config` option ([#281](https://github.com/jogotcha/dufs/issues/281)) ([4ef0773](https://github.com/jogotcha/dufs/commit/4ef07737e12b4b1ca001a979ddd2fb473eda0409))
* support customize http log format ([#116](https://github.com/jogotcha/dufs/issues/116)) ([ae2f878](https://github.com/jogotcha/dufs/commit/ae2f878e62ca36e9c93da922fa8708a52d3d43c8))
* support delete operation ([dd888f1](https://github.com/jogotcha/dufs/commit/dd888f1ae85f20ef58ce8710229f71e4dea942de))
* support ecdsa tls cert ([#119](https://github.com/jogotcha/dufs/issues/119)) ([0918fb3](https://github.com/jogotcha/dufs/commit/0918fb3fe49d14568e946fac5d62026795986afd))
* support edit files ([#179](https://github.com/jogotcha/dufs/issues/179)) ([dd69734](https://github.com/jogotcha/dufs/commit/dd6973468cb14f54343e594a641d4b9411c11d25)), closes [#172](https://github.com/jogotcha/dufs/issues/172)
* support gracefully shutdown server ([c3dd0f0](https://github.com/jogotcha/dufs/commit/c3dd0f0ec5e260faba7cb7fc54aacc080b07a07a))
* support hashed password ([#283](https://github.com/jogotcha/dufs/issues/283)) ([d3de3db](https://github.com/jogotcha/dufs/commit/d3de3db0d9d6403ae94e4372654d2ac0e594be68))
* support hiding folders with --hidden ([#73](https://github.com/jogotcha/dufs/issues/73)) ([eb7a536](https://github.com/jogotcha/dufs/commit/eb7a536a3f471f0102d11065e20b3a6f8df0f139))
* support ipv6 ([#25](https://github.com/jogotcha/dufs/issues/25)) ([63a7b53](https://github.com/jogotcha/dufs/commit/63a7b530bb99bd291567dbf919256f8c35ba96da))
* support multipart ranges ([#535](https://github.com/jogotcha/dufs/issues/535)) ([eda9769](https://github.com/jogotcha/dufs/commit/eda9769b2a0fcd72f26e3b67542d1f361dd0f046))
* support new file ([#180](https://github.com/jogotcha/dufs/issues/180)) ([a61fda6](https://github.com/jogotcha/dufs/commit/a61fda6e80d70e5cd7ded9a9b33a9ca373c2b239))
* support noscript fallback ([#602](https://github.com/jogotcha/dufs/issues/602)) ([089d30c](https://github.com/jogotcha/dufs/commit/089d30c5a58775dea5710d08c87692df5ccc4661))
* support path prefix ([d208b5c](https://github.com/jogotcha/dufs/commit/d208b5cb6ba4331d7f1c58275a579316bb325b80))
* support range requests ([ed7f5e4](https://github.com/jogotcha/dufs/commit/ed7f5e425a7e18a75d538b240a236287c86b8d8a))
* support render-index/render-spa ([830d334](https://github.com/jogotcha/dufs/commit/830d3343f407ac3773c664f3ca43c51eb722df71)), closes [#5](https://github.com/jogotcha/dufs/issues/5)
* support searching ([586b209](https://github.com/jogotcha/dufs/commit/586b209c89bc333fa5f73a47c163be83a420e5bc))
* support sort by name, mtime, size ([#128](https://github.com/jogotcha/dufs/issues/128)) ([31c832a](https://github.com/jogotcha/dufs/commit/31c832a7426f487dd047771a5bfe534b33f72f4d))
* support tls ([e2d7f99](https://github.com/jogotcha/dufs/commit/e2d7f996c77133ea265e936c5464b6b459cd1c78))
* support tls-key in pkcs[#8](https://github.com/jogotcha/dufs/issues/8) format ([#35](https://github.com/jogotcha/dufs/issues/35)) ([6b01c14](https://github.com/jogotcha/dufs/commit/6b01c143d9e870d2b2851fb6a7e2d041d0270867))
* support unix sockets ([#145](https://github.com/jogotcha/dufs/issues/145)) ([6ebf619](https://github.com/jogotcha/dufs/commit/6ebf619430dff7d214cd2b72add134851629b67e))
* support webdav ([#10](https://github.com/jogotcha/dufs/issues/10)) ([0a64762](https://github.com/jogotcha/dufs/commit/0a64762df4dccc3ea4dd2c5a9126994b15d7d57b))
* supports resumable uploads ([#343](https://github.com/jogotcha/dufs/issues/343)) ([ee21894](https://github.com/jogotcha/dufs/commit/ee21894452302afffe691adb5542a2b7a1db00dd))
* tls handshake timeout ([#368](https://github.com/jogotcha/dufs/issues/368)) ([d66c9de](https://github.com/jogotcha/dufs/commit/d66c9de8c838cc1d7b03012a3798d8950affabf2))
* tolerate the absence of mtime ([#559](https://github.com/jogotcha/dufs/issues/559)) ([4fbdec2](https://github.com/jogotcha/dufs/commit/4fbdec2878343267f374cbe461302476ab82f8cd))
* ui hidden root dirname ([#58](https://github.com/jogotcha/dufs/issues/58)) ([db71f75](https://github.com/jogotcha/dufs/commit/db71f75236e4893463d0a227d27a97f34bf6f30a)), closes [#56](https://github.com/jogotcha/dufs/issues/56)
* ui improves the login experience ([#182](https://github.com/jogotcha/dufs/issues/182)) ([6d9758c](https://github.com/jogotcha/dufs/commit/6d9758c71d2827bfcc2c36bdf1bd29c2cfb9b773))
* ui supports creating folder ([#91](https://github.com/jogotcha/dufs/issues/91)) ([b6729a3](https://github.com/jogotcha/dufs/commit/b6729a3d641d0dac5cb560e9add86cfd9f322fcc))
* ui supports move folder/file to new path ([#92](https://github.com/jogotcha/dufs/issues/92)) ([c6541b1](https://github.com/jogotcha/dufs/commit/c6541b1c365916ceb0d0acd3f34de780d559fae6))
* ui supports view file ([#301](https://github.com/jogotcha/dufs/issues/301)) ([073b098](https://github.com/jogotcha/dufs/commit/073b09811138e8079f710d80486b1ed31c7110d7))
* **ui:** add table row hover ([#115](https://github.com/jogotcha/dufs/issues/115)) ([277d9d2](https://github.com/jogotcha/dufs/commit/277d9d22d44fd1922b4ea53be287e656f23130b4))
* unzip zip file when unload ([62696b4](https://github.com/jogotcha/dufs/commit/62696b45fde4baef6fa09b4e82470eefca1b5149))
* upgrade to hyper 1.0 ([#321](https://github.com/jogotcha/dufs/issues/321)) ([270cc0c](https://github.com/jogotcha/dufs/commit/270cc0cba20adde0eca09a5ab963c86d781c1b25))
* use custom logger with timestamp in rfc3339 ([#67](https://github.com/jogotcha/dufs/issues/67)) ([7f062b6](https://github.com/jogotcha/dufs/commit/7f062b6705665c906f2386ef8dc340026d1e3f4f))
* use digest auth ([#14](https://github.com/jogotcha/dufs/issues/14)) ([2f40313](https://github.com/jogotcha/dufs/commit/2f40313a5458c4b08b9b665a3621d2295cddd46c))
* use env var for args ([#170](https://github.com/jogotcha/dufs/issues/170)) ([fea9bf9](https://github.com/jogotcha/dufs/commit/fea9bf988a4d9c2ba96fa209f1448d193df2a41b)), closes [#160](https://github.com/jogotcha/dufs/issues/160)
* use feature to conditional support tls ([#77](https://github.com/jogotcha/dufs/issues/77)) ([6554c1c](https://github.com/jogotcha/dufs/commit/6554c1c308fc5d316a71484cbb186e60f1739608))
* webui displays subdirectory items ([#457](https://github.com/jogotcha/dufs/issues/457)) ([bb5a556](https://github.com/jogotcha/dufs/commit/bb5a5564b4bf98d0a8422829599d1e5ffa353bcb))
* webui editing support multiple encodings ([#197](https://github.com/jogotcha/dufs/issues/197)) ([e43554b](https://github.com/jogotcha/dufs/commit/e43554b7952453463d894097f443b3387f5ebb9d))
* webui support logout ([#439](https://github.com/jogotcha/dufs/issues/439)) ([4bf92cc](https://github.com/jogotcha/dufs/commit/4bf92cc47a14a54a209e3b0757914a9137a4dc50))
* zip browsing ([#1](https://github.com/jogotcha/dufs/issues/1)) ([c06598f](https://github.com/jogotcha/dufs/commit/c06598fd2a3bd7e66322c71676318c7baab00930))


### Bug Fixes

* allow all cors headers and methods ([#225](https://github.com/jogotcha/dufs/issues/225)) ([27c269d](https://github.com/jogotcha/dufs/commit/27c269d6a07c52b2bd8b33bf2bde698c97c5ab1b))
* auth failed if password contains `:` ([#449](https://github.com/jogotcha/dufs/issues/449)) ([c500ce7](https://github.com/jogotcha/dufs/commit/c500ce7accef17ab6dc78af6e0e5eb39de5e7168))
* auth logic ([#224](https://github.com/jogotcha/dufs/issues/224)) ([57b4a74](https://github.com/jogotcha/dufs/commit/57b4a7427930377009cf3cd1ec402ad75e4cad3d))
* auth not works with --path-prefix ([#138](https://github.com/jogotcha/dufs/issues/138)) ([dbf2de9](https://github.com/jogotcha/dufs/commit/dbf2de9cb9216c554788b1bcc01d3d44d3c4ad23)), closes [#137](https://github.com/jogotcha/dufs/issues/137)
* auth precedence ([#325](https://github.com/jogotcha/dufs/issues/325)) ([77f86a4](https://github.com/jogotcha/dufs/commit/77f86a4c60b19472a69cf03f81cb2fa1d66a9cdf))
* auto delete half-uploaded files ([#280](https://github.com/jogotcha/dufs/issues/280)) ([8b4cab1](https://github.com/jogotcha/dufs/commit/8b4cab1e691f46c0c0cbbcae8dc0b4e033114706))
* basic auth sometimes does not work ([#194](https://github.com/jogotcha/dufs/issues/194)) ([c92e45f](https://github.com/jogotcha/dufs/commit/c92e45f2da629140cdfc806e333ceeefc25dd613))
* broken ui ([48c3c7d](https://github.com/jogotcha/dufs/commit/48c3c7ded67cb9e5e82a74cec854c3d00deb4824))
* cannot upload ([#32](https://github.com/jogotcha/dufs/issues/32)) ([a84c3b3](https://github.com/jogotcha/dufs/commit/a84c3b353d65be680fcea60336104920867b5179)), closes [#31](https://github.com/jogotcha/dufs/issues/31)
* cannot upload in root ([d9a9171](https://github.com/jogotcha/dufs/commit/d9a917176ae5ee96465bc57daf9a66a222cca174))
* caught 500 if no permission to access dir ([35ed439](https://github.com/jogotcha/dufs/commit/35ed4394df5673e537c8b59d8563d59e34afbf4c)), closes [#4](https://github.com/jogotcha/dufs/issues/4)
* caught server error when symlink broken ([61731d2](https://github.com/jogotcha/dufs/commit/61731d22e334ae47c96cc028e66f372bf882e259))
* clear search input also clear query ([#178](https://github.com/jogotcha/dufs/issues/178)) ([111103f](https://github.com/jogotcha/dufs/commit/111103f26b501605639909ff74ccd393f50c5d22)), closes [#161](https://github.com/jogotcha/dufs/issues/161)
* corrupted zip when downloading large folders ([#337](https://github.com/jogotcha/dufs/issues/337)) ([0ac0c04](https://github.com/jogotcha/dufs/commit/0ac0c048eca56b2f19a98430691b208c1a196f17))
* cors allow-request-header add content-type ([#184](https://github.com/jogotcha/dufs/issues/184)) ([6dcb4dc](https://github.com/jogotcha/dufs/commit/6dcb4dcd76252e4a36fbb1c8f7f52ae4a0da9de1))
* cors headers ([#100](https://github.com/jogotcha/dufs/issues/100)) ([4e84e6c](https://github.com/jogotcha/dufs/commit/4e84e6c5322ad2c2311ab5a21733bf7d5eee787c))
* ctrl+c not exit sometimes ([882a9ae](https://github.com/jogotcha/dufs/commit/882a9ae7168f17f9d2bf7877e631a299a558787c))
* decodeURI searching string ([#61](https://github.com/jogotcha/dufs/issues/61)) ([069cb64](https://github.com/jogotcha/dufs/commit/069cb64889563da2c320b36837dbacbb6b545877))
* don't search on empty query string ([#140](https://github.com/jogotcha/dufs/issues/140)) ([1a9990f](https://github.com/jogotcha/dufs/commit/1a9990f04e5a0aaefb330873f8daece896a0f2ad))
* downloaded zip file has no.zip ext in firefox ([412d42e](https://github.com/jogotcha/dufs/commit/412d42e338b00936b78c2b1cb40fc83043cccb80)), closes [#2](https://github.com/jogotcha/dufs/issues/2)
* encode webdav href as uri ([#28](https://github.com/jogotcha/dufs/issues/28)) ([8d03ec1](https://github.com/jogotcha/dufs/commit/8d03ec151a539522a33d66721b9c5fd1c249e7eb))
* ensure symlink inside serve root ([#670](https://github.com/jogotcha/dufs/issues/670)) ([a118c13](https://github.com/jogotcha/dufs/commit/a118c1348e07bf8312e2ea5b7edabd3b2dca0e11))
* escape filename ([#21](https://github.com/jogotcha/dufs/issues/21)) ([f138915](https://github.com/jogotcha/dufs/commit/f138915f20fd019dac8d0c0b3acb50dbf319bd21)), closes [#19](https://github.com/jogotcha/dufs/issues/19)
* escape filename in ?simple output ([#669](https://github.com/jogotcha/dufs/issues/669)) ([db7a053](https://github.com/jogotcha/dufs/commit/db7a0530a29bbe3dd8105f1f469784d606f9cba5))
* escape name contains html escape code ([#65](https://github.com/jogotcha/dufs/issues/65)) ([ea8b9e9](https://github.com/jogotcha/dufs/commit/ea8b9e9cce3fa26dc8cff137cdd06cc00f17e4a6))
* escape path-prefix/url-prefix different ([b0cc901](https://github.com/jogotcha/dufs/commit/b0cc901416ad667bdf541a476f8ff8bdb54643e3))
* filename xml escaping ([ce154d9](https://github.com/jogotcha/dufs/commit/ce154d9ebca4012d88e7523dfaef6b825359b9b6))
* follow symlinks when searching/archiving ([#572](https://github.com/jogotcha/dufs/issues/572)) ([8a92a0c](https://github.com/jogotcha/dufs/commit/8a92a0cf1a1f2cd2341cb118e811d5f12177a8c3))
* garbled characters caused by atob ([#422](https://github.com/jogotcha/dufs/issues/422)) ([7aba3fe](https://github.com/jogotcha/dufs/commit/7aba3fe0b630f8480e89eac52c0a632e94772c5b))
* guard req and destination path ([#359](https://github.com/jogotcha/dufs/issues/359)) ([3c75a9c](https://github.com/jogotcha/dufs/commit/3c75a9c4ccb6b24bb7dc5b54c8c471282f014259))
* head div overlap main contents when wrap ([#386](https://github.com/jogotcha/dufs/issues/386)) ([fe23585](https://github.com/jogotcha/dufs/commit/fe2358506d21b791abcba9e03cefc5c299397728))
* hidden don't works on some files ([#188](https://github.com/jogotcha/dufs/issues/188)) ([0e12b28](https://github.com/jogotcha/dufs/commit/0e12b285cd6d0d2cfceecfcaf67b71b5df315dbe))
* hide path by ext name ([#126](https://github.com/jogotcha/dufs/issues/126)) ([3ae75d3](https://github.com/jogotcha/dufs/commit/3ae75d35586638e9de2aa653868a2f0210ba5642))
* incorrect dir size due to hidden files ([#529](https://github.com/jogotcha/dufs/issues/529)) ([d255f13](https://github.com/jogotcha/dufs/commit/d255f1376a23f24bce725a7f7c1f665f5c8f8960))
* incorrect seperator for zip archives under windows ([#577](https://github.com/jogotcha/dufs/issues/577)) ([53f064c](https://github.com/jogotcha/dufs/commit/53f064c73b07a54526472eba3c1066e3e8c90651))
* login btn does not work for readonly annoymous ([#620](https://github.com/jogotcha/dufs/issues/620)) ([4016715](https://github.com/jogotcha/dufs/commit/4016715187db5bd84c7d15ea6abcd99fd4a0a667))
* login successed but popup `Forbidden` ([#437](https://github.com/jogotcha/dufs/issues/437)) ([7d17d9c](https://github.com/jogotcha/dufs/commit/7d17d9c4154edbfafcd0dc80061ced39630c455d))
* miss file 500 ([755554d](https://github.com/jogotcha/dufs/commit/755554d3f22bd9f102ad39e014c737986f97f0ca))
* no authentication check if no auth users ([ab4ef06](https://github.com/jogotcha/dufs/commit/ab4ef06cb828e1a33d6c4e045a6db67b27a5bbfd))
* not found dir when allow_upload is false ([fc090b6](https://github.com/jogotcha/dufs/commit/fc090b693032e89dc714b2c781572eac03dd36ef))
* optimize download zip ([06d2b81](https://github.com/jogotcha/dufs/commit/06d2b81824fd6ba9ea7d38634d8b16d914430105))
* panic on PROPFIND // ([#144](https://github.com/jogotcha/dufs/issues/144)) ([8b4727c](https://github.com/jogotcha/dufs/commit/8b4727c3a4d6c6eb617ab6feb172796bd433a5a4))
* panic when bind already used port ([34bc8d4](https://github.com/jogotcha/dufs/commit/34bc8d411a2b1e377a2a76987f4c07a5e567f32e))
* permissions of unzipped files ([#84](https://github.com/jogotcha/dufs/issues/84)) ([2e6af67](https://github.com/jogotcha/dufs/commit/2e6af671ca8c0926367dd1aa23fcaf29f3eb1ed9))
* perms on `dufs -A -a @/:ro` ([#619](https://github.com/jogotcha/dufs/issues/619)) ([f8a7873](https://github.com/jogotcha/dufs/commit/f8a7873582567a85095ca9d2124b185cd3eb2ffd))
* query dir param ([8258dab](https://github.com/jogotcha/dufs/commit/8258dabe4a2b3841cb22e649824ce66cbb3a1609))
* range request ([#44](https://github.com/jogotcha/dufs/issues/44)) ([d8f7335](https://github.com/jogotcha/dufs/commit/d8f7335053d151c3db90d7f07a752688c3c7bdfd)), closes [#43](https://github.com/jogotcha/dufs/issues/43)
* remove Method::Options auth check ([#168](https://github.com/jogotcha/dufs/issues/168)) ([0000bd2](https://github.com/jogotcha/dufs/commit/0000bd27f579f52e012f0a7f231300d7c92dfb97))
* remove unzip file even failed to unzip ([07f4e7d](https://github.com/jogotcha/dufs/commit/07f4e7d0f22a6bcd7b0d32ce7ff2795ad81b06f2))
* rename --no-auth-read to --no-auth-access ([7c2449c](https://github.com/jogotcha/dufs/commit/7c2449cb1ad2d850ad81a1c1489e65878182078d))
* resolve speed bottleneck in 10G network ([#451](https://github.com/jogotcha/dufs/issues/451)) ([2cf6d39](https://github.com/jogotcha/dufs/commit/2cf6d3903224cd5b42b7d5b18dc25ee76acfd350))
* safari layout and compatibility ([#83](https://github.com/jogotcha/dufs/issues/83)) ([583117c](https://github.com/jogotcha/dufs/commit/583117c01fa79294c66ff66847a4ed3e529afc75))
* search should ignore entry path ([#235](https://github.com/jogotcha/dufs/issues/235)) ([a53411b](https://github.com/jogotcha/dufs/commit/a53411b4d67915a5e0c273823a3027d87cb9b2d2))
* send index page with content-type ([#26](https://github.com/jogotcha/dufs/issues/26)) ([5ce7bde](https://github.com/jogotcha/dufs/commit/5ce7bde05c58e6a3c8d532ba7d06f76ce5c26549))
* serve files with names containing newline char ([#328](https://github.com/jogotcha/dufs/issues/328)) ([006e03e](https://github.com/jogotcha/dufs/commit/006e03ed303c5ae516d9d03171d63e342315f0a8))
* set the STOPSIGNAL to SIGINT for Dockerfile ([f061365](https://github.com/jogotcha/dufs/commit/f061365587c3141b82338b7f89a2ef29c9923392))
* some search results missing due to broken symlinks ([#665](https://github.com/jogotcha/dufs/issues/665)) ([bc27c8c](https://github.com/jogotcha/dufs/commit/bc27c8c47948620d1deb43ae7740ec8fcb59655d))
* some typos ([f43d0b6](https://github.com/jogotcha/dufs/commit/f43d0b646db280dfcd0f24e5b6455d956fe5fecb)), closes [#6](https://github.com/jogotcha/dufs/issues/6)
* sort path ignore case ([#264](https://github.com/jogotcha/dufs/issues/264)) ([60df3b4](https://github.com/jogotcha/dufs/commit/60df3b473ca183d2efba1f9341986b267563ba08))
* status code for MKCOL on existing resource ([#142](https://github.com/jogotcha/dufs/issues/142)) ([604ccc6](https://github.com/jogotcha/dufs/commit/604ccc655606319029687c278180d078bd133666))
* strange issue that occurs only on Microsoft WebDAV ([#382](https://github.com/jogotcha/dufs/issues/382)) ([cb7d417](https://github.com/jogotcha/dufs/commit/cb7d417fd38b1ca4ba1ddcd607930e6d2ead6e8e))
* table row hover highlighting in dark mode ([#122](https://github.com/jogotcha/dufs/issues/122)) ([a489c56](https://github.com/jogotcha/dufs/commit/a489c5647ad4fced7f771b8b18f39cceda646db1))
* timestamp format of getlastmodified in dav xml ([#366](https://github.com/jogotcha/dufs/issues/366)) ([1c41db0](https://github.com/jogotcha/dufs/commit/1c41db0c2d78413b03c47ae91089a34fb49d9b89))
* typo __ASSERTS_PREFIX__ ([#252](https://github.com/jogotcha/dufs/issues/252)) ([7f83de7](https://github.com/jogotcha/dufs/commit/7f83de765a4e709597aa38a8a8a1cad1d3701a27))
* ui path table show move action ([#219](https://github.com/jogotcha/dufs/issues/219)) ([4622c48](https://github.com/jogotcha/dufs/commit/4622c48120099488a6e6213940e3fc2d08019530))
* ui readonly if no write perm ([#258](https://github.com/jogotcha/dufs/issues/258)) ([9545fb6](https://github.com/jogotcha/dufs/commit/9545fb6e37edd1d91af0428cc715658b49a32bed))
* ui refresh page after login ([#230](https://github.com/jogotcha/dufs/issues/230)) ([8be545d](https://github.com/jogotcha/dufs/commit/8be545d3da1202e4fe8105a8a922bce547c357e2))
* ui set default max uploading to 1 ([#220](https://github.com/jogotcha/dufs/issues/220)) ([8d7c1fb](https://github.com/jogotcha/dufs/commit/8d7c1fbf537b0641e5946ef49108fc042e07d2da))
* ui show user-name next to the user-icon ([#278](https://github.com/jogotcha/dufs/issues/278)) ([6766e0d](https://github.com/jogotcha/dufs/commit/6766e0d4376e6e8f0cc0fe665849cf181b84eead))
* **ui:** file path contains special charactors ([#114](https://github.com/jogotcha/dufs/issues/114)) ([c62926d](https://github.com/jogotcha/dufs/commit/c62926d19c4b1fad7631780077b6844bf4caa042))
* unable to start if config file omit bind/port fields ([#294](https://github.com/jogotcha/dufs/issues/294)) ([afdfde0](https://github.com/jogotcha/dufs/commit/afdfde01f02b64b119a8ff38a93fffda09103b97))
* unexpect stack overflow when searching a lot ([#87](https://github.com/jogotcha/dufs/issues/87)) ([1e0cdaf](https://github.com/jogotcha/dufs/commit/1e0cdafbcfb6b7823591f7aee9cbc83d23ea4bd9))
* unexpected public auth asking for login info ([#583](https://github.com/jogotcha/dufs/issues/583)) ([f8b69f4](https://github.com/jogotcha/dufs/commit/f8b69f4df8ee4eb8ca7ac94beb8d46e3644374aa))
* unzip override existed file in uploadonly mode ([6a097e0](https://github.com/jogotcha/dufs/commit/6a097e0496aefb76c16f6834772faadf204bfb31))
* update send_zip_edit to handle edit/view mode based on query parameters ([57b7744](https://github.com/jogotcha/dufs/commit/57b77444276a77f71832dbdf5b26e18c6f5c136b))
* upload more than 100 files in directory ([#317](https://github.com/jogotcha/dufs/issues/317)) ([cd84dff](https://github.com/jogotcha/dufs/commit/cd84dff87f1ffd0938e7c54762b104ff0116d02a))
* URL-encoded filename when downloading in safari ([#203](https://github.com/jogotcha/dufs/issues/203)) ([fb5b50f](https://github.com/jogotcha/dufs/commit/fb5b50f059d39144c2425c75f5d523622d63aac9))
* use DUFS_CONFIG to specify the config file path ([#286](https://github.com/jogotcha/dufs/issues/286)) ([a476c15](https://github.com/jogotcha/dufs/commit/a476c15a09aaf49e95b193dcdb9a99dd71c81b1e))
* verify token length ([#627](https://github.com/jogotcha/dufs/issues/627)) ([db75ba4](https://github.com/jogotcha/dufs/commit/db75ba4357d80c38fbe4b4473c738fa2f8e0bcac))
* webdav only see public folder even logging in ([#231](https://github.com/jogotcha/dufs/issues/231)) ([6be36b8](https://github.com/jogotcha/dufs/commit/6be36b8e51830e69f66510193096e79e59c74474))
* webdav propfind dir with slash ([#42](https://github.com/jogotcha/dufs/issues/42)) ([c7d42a3](https://github.com/jogotcha/dufs/commit/c7d42a3f1cdeec115007a7264dcb970113a550a9))
* webui can't handle hash property of URL well ([#515](https://github.com/jogotcha/dufs/issues/515)) ([af95ea1](https://github.com/jogotcha/dufs/commit/af95ea1cd740683e0355417cff97fb786c927cb5))
* webui formatDirSize ([#568](https://github.com/jogotcha/dufs/issues/568)) ([59685da](https://github.com/jogotcha/dufs/commit/59685da06e8797ef8089a7cf3ac72ba6bdcac68d))
* webui unexpected save-btn when file is non-editable ([#429](https://github.com/jogotcha/dufs/issues/429)) ([5d26103](https://github.com/jogotcha/dufs/commit/5d26103ea201119a6683ed709f621c52b409f01b))


### Code Refactoring

* rename --cors to --enable-cors ([#57](https://github.com/jogotcha/dufs/issues/57)) ([e66951f](https://github.com/jogotcha/dufs/commit/e66951fd1126dbce48ea8a87285786865e25402f))

## [0.45.0] - 2025-09-03

### Bug Fixes

- Perms on `dufs -A -a @/:ro` ([#619](https://github.com/sigoden/dufs/issues/619))
- Login btn does not work for readonly anonymous ([#620](https://github.com/sigoden/dufs/issues/620))
- Verify token length ([#627](https://github.com/sigoden/dufs/issues/627))

### Features

- Make dir urls inherit `?noscript` params ([#614](https://github.com/sigoden/dufs/issues/614))
- Log decoded uri ([#615](https://github.com/sigoden/dufs/issues/615))

## [0.44.0] - 2025-08-02

### Bug Fixes

- No authentication check if no auth users ([#497](https://github.com/sigoden/dufs/issues/497))
- Webui can't handle hash property of URL well ([#515](https://github.com/sigoden/dufs/issues/515))
- Incorrect dir size due to hidden files ([#529](https://github.com/sigoden/dufs/issues/529))
- Webui formatDirSize ([#568](https://github.com/sigoden/dufs/issues/568))
- Follow symlinks when searching/archiving ([#572](https://github.com/sigoden/dufs/issues/572))
- Incorrect separator for zip archives under windows ([#577](https://github.com/sigoden/dufs/issues/577))
- Unexpected public auth asking for login info ([#583](https://github.com/sigoden/dufs/issues/583))

### Features

- Higher perm auth path shadows lower one ([#521](https://github.com/sigoden/dufs/issues/521))
- Add cache-control:no-cache while sending file and index ([#528](https://github.com/sigoden/dufs/issues/528))
- Support multipart ranges ([#535](https://github.com/sigoden/dufs/issues/535))
- Limit sub directory item counting ([#556](https://github.com/sigoden/dufs/issues/556))
- Tolerate the absence of mtime ([#559](https://github.com/sigoden/dufs/issues/559))
- Support noscript fallback ([#602](https://github.com/sigoden/dufs/issues/602))
- Support  downloading via token auth ([#603](https://github.com/sigoden/dufs/issues/603))

### Refactor

- Change description for `--allow-archive` ([#511](https://github.com/sigoden/dufs/issues/511))
- Removes clippy warnings ([#601](https://github.com/sigoden/dufs/issues/601))
- Update deps ([#604](https://github.com/sigoden/dufs/issues/604))
- Fix typos ([#605](https://github.com/sigoden/dufs/issues/605))

## [0.43.0] - 2024-11-04

### Bug Fixes

- Auth failed if password contains `:` ([#449](https://github.com/sigoden/dufs/issues/449))
- Resolve speed bottleneck in 10G network ([#451](https://github.com/sigoden/dufs/issues/451))

### Features

- Webui displays subdirectory items ([#457](https://github.com/sigoden/dufs/issues/457))
- Support binding abstract unix socket ([#468](https://github.com/sigoden/dufs/issues/468))
- Provide healthcheck API ([#474](https://github.com/sigoden/dufs/issues/474))

### Refactor

- Do not show size for Dir ([#447](https://github.com/sigoden/dufs/issues/447))

## [0.42.0] - 2024-09-01

### Bug Fixes

- Garbled characters caused by atob ([#422](https://github.com/sigoden/dufs/issues/422))
- Webui unexpected save-btn when file is non-editable ([#429](https://github.com/sigoden/dufs/issues/429))
- Login succeeded but popup `Forbidden` ([#437](https://github.com/sigoden/dufs/issues/437))

### Features

- Implements remaining http cache conditionalss ([#407](https://github.com/sigoden/dufs/issues/407))
- Base64 index-data to avoid misencoding ([#421](https://github.com/sigoden/dufs/issues/421))
- Webui support logout ([#439](https://github.com/sigoden/dufs/issues/439))

### Refactor

- No inline scripts in HTML ([#391](https://github.com/sigoden/dufs/issues/391))
- Return 400 for propfind request when depth is neither 0 nor 1 ([#403](https://github.com/sigoden/dufs/issues/403))
- Remove sabredav-partialupdate from DAV res header ([#415](https://github.com/sigoden/dufs/issues/415))
- Date formatting in cache tests ([#428](https://github.com/sigoden/dufs/issues/428))
- Some query params work as flag and must not accept a value ([#431](https://github.com/sigoden/dufs/issues/431))
- Improve logout at asserts/index.js ([#440](https://github.com/sigoden/dufs/issues/440))
- Make logout works on safari ([#442](https://github.com/sigoden/dufs/issues/442))

## [0.41.0] - 2024-05-22

### Bug Fixes

- Timestamp format of getlastmodified in dav xml ([#366](https://github.com/sigoden/dufs/issues/366))
- Strange issue that occurs only on Microsoft WebDAV ([#382](https://github.com/sigoden/dufs/issues/382))
- Head div overlap main contents when wrap ([#386](https://github.com/sigoden/dufs/issues/386))

### Features

- Tls handshake timeout ([#368](https://github.com/sigoden/dufs/issues/368))
- Add api to get the hash of a file ([#375](https://github.com/sigoden/dufs/issues/375))
- Add log-file option ([#383](https://github.com/sigoden/dufs/issues/383))

### Refactor

- Digest_auth related tests ([#372](https://github.com/sigoden/dufs/issues/372))
- Add fixed-width numerals to date and size on file list page ([#378](https://github.com/sigoden/dufs/issues/378))

## [0.40.0] - 2024-02-13

### Bug Fixes

- Guard req and destination path ([#359](https://github.com/sigoden/dufs/issues/359))

### Features

- Revert supporting for forbidden permission ([#352](https://github.com/sigoden/dufs/issues/352))

### Refactor

- Do not try to bind ipv6 if no ipv6 ([#348](https://github.com/sigoden/dufs/issues/348))
- Improve invalid auth ([#356](https://github.com/sigoden/dufs/issues/356))
- Improve resolve_path and handle_assets, abandon guard_path ([#360](https://github.com/sigoden/dufs/issues/360))

## [0.39.0] - 2024-01-11

### Bug Fixes

- Upload more than 100 files in directory ([#317](https://github.com/sigoden/dufs/issues/317))
- Auth precedence ([#325](https://github.com/sigoden/dufs/issues/325))
- Serve files with names containing newline char ([#328](https://github.com/sigoden/dufs/issues/328))
- Corrupted zip when downloading large folders ([#337](https://github.com/sigoden/dufs/issues/337))

### Features

- Empty search `?q=` list all paths ([#311](https://github.com/sigoden/dufs/issues/311))
- Add `--compress` option ([#319](https://github.com/sigoden/dufs/issues/319))
- Upgrade to hyper 1.0 ([#321](https://github.com/sigoden/dufs/issues/321))
- Auth supports forbidden permissions ([#329](https://github.com/sigoden/dufs/issues/329))
- Supports resumable uploads ([#343](https://github.com/sigoden/dufs/issues/343))

### Refactor

- Change the format of www-authenticate ([#312](https://github.com/sigoden/dufs/issues/312))
- Change the value name of `--config` ([#313](https://github.com/sigoden/dufs/issues/313))
- Optimize http range parsing and handling ([#323](https://github.com/sigoden/dufs/issues/323))
- Propfind with auth no need to list all ([#344](https://github.com/sigoden/dufs/issues/344))

## [0.38.0] - 2023-11-28

### Bug Fixes

- Unable to start if config file omit bind/port fields ([#294](https://github.com/sigoden/dufs/issues/294))

### Features

- Password can contain `:` `@` `|` ([#297](https://github.com/sigoden/dufs/issues/297))
- Deprecate the use of `|` to separate auth rules ([#298](https://github.com/sigoden/dufs/issues/298))
- More flexible config values ([#299](https://github.com/sigoden/dufs/issues/299))
- Ui supports view file ([#301](https://github.com/sigoden/dufs/issues/301))

### Refactor

- Take improvements from the edge browser ([#289](https://github.com/sigoden/dufs/issues/289))
- Ui change the cursor for upload-btn to a pointer ([#291](https://github.com/sigoden/dufs/issues/291))
- Ui improve uploading progress ([#296](https://github.com/sigoden/dufs/issues/296))

## [0.37.1] - 2023-11-08

### Bug Fixes

- Use DUFS_CONFIG to specify the config file path ([#286](https://github.com/sigoden/dufs/issues/286)

## [0.37.0] - 2023-11-08

### Bug Fixes

- Sort path ignore case ([#264](https://github.com/sigoden/dufs/issues/264))
- Ui show user-name next to the user-icon ([#278](https://github.com/sigoden/dufs/issues/278))
- Auto delete half-uploaded files ([#280](https://github.com/sigoden/dufs/issues/280))

### Features

- Deprecate `--auth-method`,  as both options are available ([#279](https://github.com/sigoden/dufs/issues/279))
- Support config file with `--config` option ([#281](https://github.com/sigoden/dufs/issues/281))
- Support hashed password ([#283](https://github.com/sigoden/dufs/issues/283))

### Refactor

- Remove one clone on `assets_prefix` ([#270](https://github.com/sigoden/dufs/issues/270))
- Optimize tests
- Improve code quality ([#282](https://github.com/sigoden/dufs/issues/282))

## [0.36.0] - 2023-08-24

### Bug Fixes

- Ui readonly if no write perm ([#258](https://github.com/sigoden/dufs/issues/258))

### Testing

- Remove dependency on native tls ([#255](https://github.com/sigoden/dufs/issues/255))

## [0.35.0] - 2023-08-14

### Bug Fixes

- Search should ignore entry path ([#235](https://github.com/sigoden/dufs/issues/235))
- Typo __ASSERTS_PREFIX__ ([#252](https://github.com/sigoden/dufs/issues/252))

### Features

- Sort by type first, then sort by name/mtime/size ([#241](https://github.com/sigoden/dufs/issues/241))

## [0.34.2] - 2023-06-05

### Bug Fixes

- Ui refresh page after login ([#230](https://github.com/sigoden/dufs/issues/230))
- Webdav only see public folder even logging in ([#231](https://github.com/sigoden/dufs/issues/231))

## [0.34.1] - 2023-06-02

### Bug Fixes

- Auth logic ([#224](https://github.com/sigoden/dufs/issues/224))
- Allow all cors headers and methods ([#225](https://github.com/sigoden/dufs/issues/225))

### Refactor

- Ui checkAuth ([#226](https://github.com/sigoden/dufs/issues/226))

## [0.34.0] - 2023-06-01

### Bug Fixes

- URL-encoded filename when downloading in safari ([#203](https://github.com/sigoden/dufs/issues/203))
- Ui path table show move action ([#219](https://github.com/sigoden/dufs/issues/219))
- Ui set default max uploading to 1 ([#220](https://github.com/sigoden/dufs/issues/220))

### Features

- Webui editing support multiple encodings ([#197](https://github.com/sigoden/dufs/issues/197))
- Add timestamp metadata to generated zip file ([#204](https://github.com/sigoden/dufs/issues/204))
- Show precise file size with decimal ([#210](https://github.com/sigoden/dufs/issues/210))
- [**breaking**] New auth ([#218](https://github.com/sigoden/dufs/issues/218))

### Refactor

- Cli positional rename root => SERVE_PATH([#215](https://github.com/sigoden/dufs/issues/215))

## [0.33.0] - 2023-03-17

### Bug Fixes

- Cors allow-request-header add content-type ([#184](https://github.com/sigoden/dufs/issues/184))
- Hidden don't works on some files ([#188](https://github.com/sigoden/dufs/issues/188))
- Basic auth sometimes does not work ([#194](https://github.com/sigoden/dufs/issues/194))

### Features

- Guess plain text encoding then set content-type charset ([#186](https://github.com/sigoden/dufs/issues/186))

### Refactor

- Improve error handle ([#195](https://github.com/sigoden/dufs/issues/195))

## [0.32.0] - 2023-02-22

### Bug Fixes

- Set the STOPSIGNAL to SIGINT for Dockerfile
- Remove Method::Options auth check ([#168](https://github.com/sigoden/dufs/issues/168))
- Clear search input also clear query ([#178](https://github.com/sigoden/dufs/issues/178))

### Features

- [**breaking**] Add option --allow-archive ([#152](https://github.com/sigoden/dufs/issues/152))
- Use env var for args ([#170](https://github.com/sigoden/dufs/issues/170))
- Hiding only directories instead of files ([#175](https://github.com/sigoden/dufs/issues/175))
- API to search and list directories ([#177](https://github.com/sigoden/dufs/issues/177))
- Support edit files ([#179](https://github.com/sigoden/dufs/issues/179))
- Support new file ([#180](https://github.com/sigoden/dufs/issues/180))
- Ui improves the login experience ([#182](https://github.com/sigoden/dufs/issues/182))

## [0.31.0] - 2022-11-11

### Bug Fixes

- Auth not works with --path-prefix ([#138](https://github.com/sigoden/dufs/issues/138))
- Don't search on empty query string ([#140](https://github.com/sigoden/dufs/issues/140))
- Status code for MKCOL on existing resource ([#142](https://github.com/sigoden/dufs/issues/142))
- Panic on PROPFIND // ([#144](https://github.com/sigoden/dufs/issues/144))

### Features

- Support unix sockets ([#145](https://github.com/sigoden/dufs/issues/145))

## [0.30.0] - 2022-09-09

### Bug Fixes

- Hide path by ext name ([#126](https://github.com/sigoden/dufs/issues/126))

### Features

- Support sort by name, mtime, size ([#128](https://github.com/sigoden/dufs/issues/128))
- Add --assets options to override assets ([#134](https://github.com/sigoden/dufs/issues/134))

## [0.29.0] - 2022-08-03

### Bug Fixes

- Table row hover highlighting in dark mode ([#122](https://github.com/sigoden/dufs/issues/122))

### Features

- Support ecdsa tls cert ([#119](https://github.com/sigoden/dufs/issues/119))

## [0.28.0] - 2022-08-01

### Bug Fixes

- File path contains special characters ([#114](https://github.com/sigoden/dufs/issues/114))

### Features

- Add table row hover ([#115](https://github.com/sigoden/dufs/issues/115))
- Support customize http log format ([#116](https://github.com/sigoden/dufs/issues/116))

## [0.27.0] - 2022-07-25

### Features

- Improve hidden to support glob ([#108](https://github.com/sigoden/dufs/issues/108))
- Adjust digest auth timeout to 1day ([#110](https://github.com/sigoden/dufs/issues/110))

## [0.26.0] - 2022-07-11

### Bug Fixes

- Cors headers ([#100](https://github.com/sigoden/dufs/issues/100))

### Features

- Make --path-prefix works on serving single file ([#102](https://github.com/sigoden/dufs/issues/102))

## [0.25.0] - 2022-07-06

### Features

- Ui supports creating folder ([#91](https://github.com/sigoden/dufs/issues/91))
- Ui supports move folder/file to new path ([#92](https://github.com/sigoden/dufs/issues/92))
- Check permission on move/copy destination ([#93](https://github.com/sigoden/dufs/issues/93))
- Add completions ([#97](https://github.com/sigoden/dufs/issues/97))
- Limit the number of concurrent uploads ([#98](https://github.com/sigoden/dufs/issues/98))

## [0.24.0] - 2022-07-02

### Bug Fixes

- Unexpected stack overflow when searching a lot ([#87](https://github.com/sigoden/dufs/issues/87))

### Features

- Allow search with --render-try-index ([#88](https://github.com/sigoden/dufs/issues/88))

## [0.23.1] - 2022-06-30

### Bug Fixes

- Safari layout and compatibility ([#83](https://github.com/sigoden/dufs/issues/83))
- Permissions of unzipped files ([#84](https://github.com/sigoden/dufs/issues/84))

## [0.23.0] - 2022-06-29

### Features

- Use feature to conditional support tls ([#77](https://github.com/sigoden/dufs/issues/77))

### Ci

- Support more platforms ([#76](https://github.com/sigoden/dufs/issues/76))

## [0.22.0] - 2022-06-26

### Features

- Support hiding folders with --hidden ([#73](https://github.com/sigoden/dufs/issues/73))

## [0.21.0] - 2022-06-23

### Bug Fixes

- Escape name contains html escape code ([#65](https://github.com/sigoden/dufs/issues/65))

### Features

- Use custom logger with timestamp in rfc3339 ([#67](https://github.com/sigoden/dufs/issues/67))

### Refactor

- Split css/js from index.html ([#68](https://github.com/sigoden/dufs/issues/68))

## [0.20.0] - 2022-06-20

### Bug Fixes

- DecodeURI searching string ([#61](https://github.com/sigoden/dufs/issues/61))

### Features

- Added basic auth ([#60](https://github.com/sigoden/dufs/issues/60))
- Add option --allow-search ([#62](https://github.com/sigoden/dufs/issues/62))

## [0.19.0] - 2022-06-19

### Features

- [**breaking**] Path level access control ([#52](https://github.com/sigoden/dufs/issues/52))
- Serve single file ([#54](https://github.com/sigoden/dufs/issues/54))
- Ui hidden root dirname ([#58](https://github.com/sigoden/dufs/issues/58))
- Reactive webpage ([#51](https://github.com/sigoden/dufs/issues/51))
- [**breaking**] Rename to dufs ([#59](https://github.com/sigoden/dufs/issues/59))

### Refactor

- [**breaking**] Rename --cors to --enable-cors ([#57](https://github.com/sigoden/dufs/issues/57))

## [0.18.0] - 2022-06-18

### Features

- Add option --render-try-index ([#47](https://github.com/sigoden/dufs/issues/47))
- Add slash to end of dir href

## [0.17.1] - 2022-06-16

### Bug Fixes

- Range request ([#44](https://github.com/sigoden/dufs/issues/44))

## [0.17.0] - 2022-06-15

### Bug Fixes

- Webdav propfind dir with slash ([#42](https://github.com/sigoden/dufs/issues/42))

### Features

- Listen both ipv4 and ipv6 by default ([#40](https://github.com/sigoden/dufs/issues/40))

### Refactor

- Trivial changes ([#41](https://github.com/sigoden/dufs/issues/41))

## [0.16.0] - 2022-06-12

### Features

- Implement head method ([#33](https://github.com/sigoden/dufs/issues/33))
- Display upload speed and time left ([#34](https://github.com/sigoden/dufs/issues/34))
- Support tls-key in pkcs#8 format ([#35](https://github.com/sigoden/dufs/issues/35))
- Options method return status 200

### Testing

- Add integration tests ([#36](https://github.com/sigoden/dufs/issues/36))

## [0.15.1] - 2022-06-11

### Bug Fixes

- Cannot upload ([#32](https://github.com/sigoden/dufs/issues/32))

## [0.15.0] - 2022-06-10

### Bug Fixes

- Encode webdav href as uri ([#28](https://github.com/sigoden/dufs/issues/28))
- Query dir param

### Features

- Add basic dark theme ([#29](https://github.com/sigoden/dufs/issues/29))
- Add empty state placeholder to page([#30](https://github.com/sigoden/dufs/issues/30))

## [0.14.0] - 2022-06-07

### Bug Fixes

- Send index page with content-type ([#26](https://github.com/sigoden/dufs/issues/26))

### Features

- Support ipv6 ([#25](https://github.com/sigoden/dufs/issues/25))
- Add favicon ([#27](https://github.com/sigoden/dufs/issues/27))

## [0.13.2] - 2022-06-06

### Bug Fixes

- Filename xml escaping
- Escape path-prefix/url-prefix different

## [0.13.1] - 2022-06-05

### Bug Fixes

- Escape filename ([#21](https://github.com/sigoden/dufs/issues/21))

### Refactor

- Use logger ([#22](https://github.com/sigoden/dufs/issues/22))

## [0.13.0] - 2022-06-05

### Bug Fixes

- Ctrl+c not exit sometimes

### Features

- Implement more webdav methods ([#13](https://github.com/sigoden/dufs/issues/13))
- Use digest auth ([#14](https://github.com/sigoden/dufs/issues/14))
- Add webdav proppatch handler ([#18](https://github.com/sigoden/dufs/issues/18))

## [0.12.1] - 2022-06-04

### Features

- Support webdav ([#10](https://github.com/sigoden/dufs/issues/10))
- Remove unzip uploaded feature ([#11](https://github.com/sigoden/dufs/issues/11))

## [0.11.0] - 2022-06-03

### Features

- Support gracefully shutdown server
- Listen 0.0.0.0 by default

## [0.10.1] - 2022-06-02

### Bug Fixes

- Panic when bind already used port

## [0.10.0] - 2022-06-02

### Bug Fixes

- Remove unzip file even failed to unzip
- Rename --no-auth-read to --no-auth-access
- Broken ui

### Documentation

- Refactor readme

### Features

- Change auth logic/options
- Improve ui

### Refactor

- Small improvement

## [0.9.0] - 2022-06-02

### Documentation

- Improve readme

### Features

- Support path prefix
- List all ifaces when listening 0.0.0.0
- Support tls

## [0.8.0] - 2022-06-01

### Bug Fixes

- Some typos
- Caught 500 if no permission to access dir

### Features

- Cli add allow-symlink option
- Add some headers to res
- Support render-index/render-spa

## [0.7.0] - 2022-05-31

### Bug Fixes

- Downloaded zip file has no.zip ext in firefox
- Unzip override existed file in uploadonly mode
- Miss file 500
- Not found dir when allow_upload is false

### Features

- Drag and drop uploads, upload folder

## [0.6.0] - 2022-05-31

### Features

- Delete confirm
- Distinct upload and delete operation
- Support range requests

### Refactor

- Improve code quality

## [0.5.0] - 2022-05-30

### Features

- Add mime and cache headers to response
- Add no-auth-read options
- Unzip zip file when unload

## [0.4.0] - 2022-05-29

### Features

- Replace --static option to --no-edit
- Add cors

## [0.3.0] - 2022-05-29

### Documentation

- Update readme demo png

### Features

- Automatically create dir while uploading
- Support searching

### Refactor

- Handler zip

### Styling

- Optimize css

## [0.2.1] - 2022-05-28

### Bug Fixes

- Cannot upload in root
- Optimize download zip

### Documentation

- Improve readme

### Features

- Aware RUST_LOG

## [0.2.0] - 2022-05-28

### Documentation

- Update demo png
- Improve readme

### Features

- Add logger
- Download folder as zip file

## [0.1.0] - 2022-05-26

### Bug Fixes

- Caught server error when symlink broken

### Documentation

- Improve readme
- Update readme

### Features

- Add basic auth and readonly mode
- Support delete operation
- Remove parent path

### Styling

- Cargo fmt
- Update index page

### Build

- Remove dev deps

### Ci

- Init ci

<!-- generated by git-cliff -->
