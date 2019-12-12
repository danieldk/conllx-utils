with import <nixpkgs> {};

mkShell {
  nativeBuildInputs = [ latest.rustChannels.stable.rust ];
}
