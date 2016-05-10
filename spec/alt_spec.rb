require 'spec_helper'

RSpec.describe Alt do
  context 'when given a path and set of possible alternate paths as stdin' do
    it 'displays the most similar alternate path from the possibles' do
      output = `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | ./alt -- spec/models/topic_link_click_spec.rb`
      expect(output).to eq('app/models/topic_link_click.rb')
    end
  end

  context 'when given a python test file' do
    it 'finds the alternate if in the possible set' do
      output = `cat spec/fixtures/python_possibles.txt | ./alt -- test/test_toaster.py`
      expect(output).to eq('toaster.py')
    end
  end

  context 'when given a python implementation file' do
    it 'finds the alternate if in the possible set' do
      output = `cat spec/fixtures/python_possibles.txt | ./alt -- toaster.py`
      expect(output).to eq('test/test_toaster.py')
    end
  end

  context 'when given the -v option' do
    it 'displays the version' do
      output = `./alt -v`
      expect(output).to eq("alt v#{Alt::VERSION}\n")
    end
  end

  context 'when given the --version option' do
    it 'displays the version' do
      output = `./alt --version`
      expect(output).to eq("alt v#{Alt::VERSION}\n")
    end
  end
end
