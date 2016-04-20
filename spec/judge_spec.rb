require 'spec_helper'

RSpec.describe Judge do
  subject { described_class }

  describe '.score' do
    it 'scores a query against a potential match' do
      query = 'spec/foo/bar/car_spec.rb'
      potential_match = 'foo/bar/zar.rb'
      score = subject.score(query, potential_match)
      expect(score).to be_within(0.0001).of(0.5714)
    end

    it 'finds the longest common substring' do
      query = 'spec/foo/bar/car_spec.rb'
      potential_match = 'foo/bar/zar.rb'
      expect(subject).to receive(:find_longest_common_substring)
        .with(query, potential_match).and_return('foo')
      subject.score(query, potential_match)
    end

    it 'computes the percentage of coverage the common substring has' do
      query = 'spec/foo/bar/car_spec.rb'
      potential_match = 'foo/bar/zar.rb'
      allow(subject).to receive(:find_longest_common_substring)
        .and_return('foo')
      expect(subject.score(query, potential_match)).to be_within(0.0001).of(3.0/14.0)
    end
  end

  describe '.find_longest_common_substring' do
    it 'returns the longest common substring' do
      query = 'spec/foo/bar/car_spec.rb'
      potential_match = 'foo/bar/zar.rb'
      expect(subject.find_longest_common_substring(query, potential_match)).to eq('foo/bar/')
    end
  end
end
