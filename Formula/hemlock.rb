class Hemlock < Formula
  desc "Vendor remote files to local folder - CLI"
  homepage "https://github.com/MadL1me/hemlock"
  url "https://github.com/MadL1me/hemlock/releases/download/v0.0.1/hemlock-mac.tar.gz"
  sha256 "955c94d237a4127851f3a6bca924f047c821a736ef7ddd845d2698680445d738"
  license "MIT"
  version "0.0.1"

  def install
    bin.install "hemlock"
  end
end