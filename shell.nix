with (import <nixpkgs> {});
mkShell {
  buildInputs = [
    mdbook
  ];
}