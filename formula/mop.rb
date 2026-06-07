class Mop < Formula
  desc "Fast macOS system cleaner CLI utility"
  homepage "https://github.com/thothlab/macos-mop"
  version "0.1.1"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/thothlab/macos-mop/releases/download/v#{version}/mop-arm64.tar.gz"
      sha256 "479d5c8e47c533c1eb757521c901db23b9f0925f5df845b2e421c89644dbff9c"
    end

    on_intel do
      url "https://github.com/thothlab/macos-mop/releases/download/v#{version}/mop-x86_64.tar.gz"
      sha256 "e3352f5a170c6c0afc35c9626a79401488fb7774f0b12135fa3b34b12445e973"
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
