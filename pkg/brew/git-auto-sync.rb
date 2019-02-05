class GitAutoSync < Formula
  desc ""
  homepage ""
  url "https://github-production-release-asset-2e65be.s3.amazonaws.com/169108351/f2802280-2935-11e9-8541-14cae583091f?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAIWNJYAX4CSVEH53A%2F20190205%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20190205T101052Z&X-Amz-Expires=300&X-Amz-Signature=e86ecce46232caf51bdeac46351c8c348bc4604345e3d01aa1a0a405fe8054b0&X-Amz-SignedHeaders=host&actor_id=10485074&response-content-disposition=attachment%3B%20filename%3Dx86_64-apple-darwin.tar.gz&response-content-type=application%2Foctet-stream"
  sha256 "e5367610234d0a83bb43446105a092cd8e88c1c1bb4b006c82b170cea30d514c"
  # depends_on "cmake" => :build

  def install
    # ENV.deparallelize  # if your formula fails when building in parallel
    # Remove unrecognized options if warned by configure
    system "./configure", "--disable-debug",
                          "--disable-dependency-tracking",
                          "--disable-silent-rules",
                          "--prefix=#{prefix}"
    # system "cmake", ".", *std_cmake_args
    system "make", "install" # if this fails, try separate make/make install steps
  end

  test do
    # `test do` will create, run in and delete a temporary directory.
    #
    # This test will fail and we won't accept that! For Homebrew/homebrew-core
    # this will need to be a test that verifies the functionality of the
    # software. Run the test with `brew test f2802280`. Options passed
    # to `brew install` such as `--HEAD` also need to be provided to `brew test`.
    #
    # The installed folder is not in the path, so use the entire path to any
    # executables being tested: `system "#{bin}/program", "do", "something"`.
    system "false"
  end
end