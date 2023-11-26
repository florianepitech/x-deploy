use kube::{Client, Config};

pub(crate) async fn connect_with_kubeconfig(file: &str) -> Client {
    // Charger la configuration depuis le fichier kubeconfig (par défaut)
    let kubeconfig = Config::infer().await?;
    // Créer un Client pour interagir avec le cluster Kubernetes
    Client::try_from(kubeconfig)?
}