class GitAutoSyncBin < Formula
    version '0.1.3'
    desc "Automatically push changes to the remote repository."
    homepage "https://github.com/jannikkeye/git-auto-sync"
  
    url "https://janisis.ams3.digitaloceanspaces.com/gas"
    end
  
    conflicts_with "git-auto-sync"
  
    def install
      bin.install "gas"
    end
  end