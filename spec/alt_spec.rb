require 'spec_helper'

RSpec.describe Alt do
  context 'when given the -v option' do
    it 'displays the version' do
      expect { Alt.new(argv: ["-v"]).main }.to output("alt v#{Alt::VERSION}\n").to_stdout
    end
  end

  context 'when given the --version option' do
    it 'displays the version' do
      expect { Alt.new(argv: ["--version"]).main }.to output("alt v#{Alt::VERSION}\n").to_stdout
    end
  end
end
