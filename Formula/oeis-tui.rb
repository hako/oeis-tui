class OeisTui < Formula
  desc "A TUI and CLI for browsing the On-Line Encyclopedia of Integer Sequences"
  homepage "https://github.com/hako/oeis-tui"
  license "MIT"
  version "1.0.0"

  on_macos do
    on_intel do
      url "https://github.com/hako/oeis-tui/releases/download/#{version}/oeis-x86_64-apple-darwin.tar.gz"
      sha256 "f9ce615c883872c3db71b3a7c509b902ea9b7866d135579cdb1b9b097e64b351"
    end

    on_arm do
      url "https://github.com/hako/oeis-tui/releases/download/#{version}/oeis-aarch64-apple-darwin.tar.gz"
      sha256 "0b8189e4337a9d5a7df89070c8aa29c31d248482e7fc33cd2f5d8397b21c60c6"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/hako/oeis-tui/releases/download/#{version}/oeis-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "fdd05624601714cfcf5c1edae40b415668a6b69e9d6df5ebd31648621676c6d8"
    end

    on_arm do
      url "https://github.com/hako/oeis-tui/releases/download/#{version}/oeis-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "ea6ca42cc4c8f2c90d97a0c8ea9ede14f3e5bfc65e5e12144545636c8199c24e"
    end
  end

  def install
    bin.install "oeis"
  end

  test do
    system "#{bin}/oeis", "--version"
  end
end
