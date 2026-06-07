class Mop < Formula
  desc "Fast macOS system cleaner CLI utility"
  homepage "https://github.com/thothlab/macos-mop"
  version "0.1.1"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/thothlab/macos-mop/releases/download/v#{version}/mop-arm64.tar.gz"
      # sha256 "PLACEHOLDER"
    end

    on_intel do
      url "https://github.com/thothlab/macos-mop/releases/download/v#{version}/mop-x86_64.tar.gz"
      # sha256 "PLACEHOLDER"
    end
  end

  def install
    if Hardware::CPU.arm?
      bin.install "mop-arm64" => "mop"
    else
      bin.install "mop-x86_64" => "mop"
    end
  end

  test do
    assert_match "mop", shell_output("#{bin}/mop --version")
  end
end
