class GitAutoSync < Formula
  desc ""
  homepage ""
  url "https://github-production-release-asset-2e65be.s3.amazonaws.com/169108351/f2802280-2935-11e9-8541-14cae583091f?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAIWNJYAX4CSVEH53A%2F20190205%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20190205T101052Z&X-Amz-Expires=300&X-Amz-Signature=e86ecce46232caf51bdeac46351c8c348bc4604345e3d01aa1a0a405fe8054b0&X-Amz-SignedHeaders=host&actor_id=10485074&response-content-disposition=attachment%3B%20filename%3Dx86_64-apple-darwin.tar.gz&response-content-type=application%2Foctet-stream"
  sha256 "e5367610234d0a83bb43446105a092cd8e88c1c1bb4b006c82b170cea30d514c"

  def install
    bin.install "git-auto-sync"
  end

  def plist; <<~EOS
    <?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0">
    <dict>
        <key>Label</key>
        <string>#{plist_name}</string>
        <key>KeepAlive</key>
        <dict>
          <key>SuccessfulExit</key>
          <false/>
        </dict>
        <key>Program</key>
        <array>
          <string>/usr/local/bin/git-auto-sync</string>
        </array>
        <key>RunAtLoad</key>
        <true/>
        <key>StandardErrorPath</key>
        <string>/usr/local/var/log/git-auto-sync.log</string>
        <key>StandardOutPath</key>
        <string>/usr/local/var/log/git-auto-sync.log</string>
      </dict>
    </plist>
    EOS
  end

  test do
    system "false"
  end
end