require 'benchmark'
require 'tmpdir'

def load_alt
  source = File.read(File.expand_path('../../alt', __FILE__))
  source = source.split('end', 2).last
  source = source.split('if $0').first
  eval(source)
end

def execute(alt)
  $stdout = File.open(File::NULL, 'w')
  alt.main
  $stdout = STDOUT
end

def setup
  root = File.realpath(Dir.mktmpdir('__test_dir__'))

  FileUtils.mkdir(File.join(root, 'spec'))
  FileUtils.mkdir(File.join(root, 'lib'))

  ('aa'..'zz').each do |name|
    FileUtils.touch(File.join(root, "spec", "rubyfile_#{name}_spec.rb"))
    FileUtils.touch(File.join(root, "lib", "rubyfile_#{name}.rb"))
  end

  root
end

load_alt
root_path = setup
alt_implementation_file = Alt.new(argv: ['lib/rubyfile_qq.rb'])
alt_test_file = Alt.new(argv: ['spec/rubyfile_qr_spec.rb'])
repeat = 1_000

Benchmark.bm do |x|
  x.report('For impl. file:') do
    Dir.chdir(root_path) do
      repeat.times { execute(alt_implementation_file) }
    end
  end

  x.report('For test file:') do
    Dir.chdir(root_path) do
      repeat.times { execute(alt_test_file) }
    end
  end
end
