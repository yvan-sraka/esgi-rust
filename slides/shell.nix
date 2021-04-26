with (import <nixpkgs> {});
mkShell {
  buildInputs = [
    pandoc
    texlive.combined.scheme-full
  ];
}