require 'benchmark'

repeat = 1_000

puts "\nRuby Implementation"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | ./alt -- lib/rubyfile_qq.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | ./alt -- spec/lib/rubyfile_qr_spec.rb` }
  end
end

puts "\nRust Implementation"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | ./target/release/alt -- lib/rubyfile_qq.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | ./target/release/alt -- spec/lib/rubyfile_qr_spec.rb` }
  end
end
