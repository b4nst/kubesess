kc() {
  config=$(kubesess ${1:+"-v "$1} context);
  # Prepend the $KUBECONFIG with new config
  export KUBECONFIG=$config:$HOME/.kube/config;
}

kcd() {
  kubesess ${1:+"-v "$1} default-context;
  # Reset the $KUBECONFIG with default
  export KUBECONFIG=$HOME/.kube/config;
}

kn() {
  config=$(kubesess ${1:+"-v "$1} namespace);
  # Prepend the $KUBECONFIG with new config
  export KUBECONFIG=$config:$HOME/.kube/config;
}

knd() {
  kubesess ${1:+"-v "$1} default-namespace;
}
