class Hemlock < Formula
  desc "Vendor remote files to local folder - CLI"
  homepage "<https://github.com/bharathvaj-ganesan/getfilesize-cli>"
  url "<https://github.com/bharathvaj-ganesan/getfilesize-cli/releases/download/v1.0.1/getfilesize.tar.gz>"
  sha256 "6c03100f5b1f8e61f5d49efab759a2efcdbcc4ac2677fcf98e7317ec660e6994"
  license "MIT"
  version "1.0.1"

  def install
    bin.install "hemlock"
  end
end