# Update signing key

`katna-update.pub` here, once it exists, is the public half of the key
that signs Katna's updates (`docs/ARCHITECTURE.md` §21.2). The Arch
package installs it as `/usr/lib/katna/update-key.pub`, and from then on
the root update helper (`packaging/arch/katna-update-helper`) installs
only packages with a good signature from its private half. CI signs each
build in `.github/workflows/arch-package.yml` with the private half, kept
in the `KATNA_UPDATE_SIGNING_KEY` repository secret, and refuses to
publish a build it cannot sign.

To set it up (the owner, once; the private key never goes in the
repository or in chat):

```sh
minisign -G -W -p katna-update.pub -s katna-update.key
```

1. Add the whole content of `katna-update.key` as the repository secret
   `KATNA_UPDATE_SIGNING_KEY` (Settings > Secrets and variables > Actions).
2. Then commit `katna-update.pub` to this folder.
3. Keep `katna-update.key` offline (a password manager), and delete the
   copy on disk.

Order matters: the secret first, then the public key. A build with the
public key and no secret fails before it publishes, so nobody gets an
update their helper would refuse.

Replacing the key: a package carrying a new public key is accepted only
if it is signed with the old one, so sign one build with the old key that
carries the new public key, then switch the secret.
