# Environment de développement

Ce répertoire contient du code [opentofu](https://opentofu.org) pour créer une VM Scaleway contenant un environnement de développement pour divers langages:

* [Emacs](https://emacs.org) de base et [neovim](https://neovim.io) pour l'édition de code,
* Fichiers de configuration tirés de [https://github.com/abailly/dotfiles](https://github.com/abailly/dotfiles)
* [GHCup](https://www.haskell.org/ghcup/) pour du Haskell
* [SBCL](https://www.sbcl.org) avec [Quicklisp](https://www.quicklisp.org/beta/) pour du Common Lisp
* [rustup](https://rustup.rs) pour du Rust
* [nvm](https://github.com/nvm-sh/nvm) pour Javascript/Typescript

## Initialisation

### Pré-requis

- Installer [OpenTofu](https://opentofu.org)
- Créer des clés d'API dans [Scaleway](https://console.scaleway.com/)
- Les stocker dans un fichier `.scaleway.tfvars` (par exemple):

  ```
  access_key       = "..."
  secret_key       = "..."
  organization_id  = "..."
  project_id       = "..."
  private_key_file = "~/.ssh/private_key_file"
  ```

### Démarrage VM

Initialisation de la configuration OpenTofu:

```
tofu init
```

Démarrage de la VM

```
tofu plan -out plan -var-file .scaleway.tfvars
tofu apply plan
```

Arrêt de la VM

```
tofu destroy -var-file .scaleway.tfvars
```

## Connexion

Pour pouvoir se connecter durant le dojo, ajoutez votre clé publique ssh au fichier [keys](./keys) en faisant une _pull request_, sous la forme:

```
<key-type> <key encoded in base64>
```

Durant la session, les participants peuvent tou·te·s se connecter sur le même terminal dans la VM avec les commandes suivantes (où `x.y.z.t` est l'adresse IP de la VM qui est communiquée avavant le dojo):

```
ssh curry@x.y.z.t
tmux a
```

## Packer

Les fichers [build-env.sh](./build-env.sh) et [build.json]()./build.json) sont utilisés pour construire l'image de la VM avec [packer](https://developer.hashicorp.com/packer)

```bash
packer build -var scw_access_key=123 -var scw_secret_key=345 build.json
```
