require 'spec_helper'

RSpec.describe Path do
  describe '.new' do
    it 'constructs a cleansed path' do
      path = described_class.new("./hoopty/doopty/doo\n")
      expect(path).to eq('hoopty/doopty/doo')
    end
  end

  describe '#test_file?' do
    context 'when path is to a test file' do
      context 'when path is to a cucumber file' do
        it 'returns true' do
          path = described_class.new('features/aouaoeu/aoeuaoeua')
          expect(path.test_file?).to eq(true)
        end
      end

      context 'when path is to a MiniTest or Test::Unit file' do
        it 'returns true' do
          path = described_class.new('test/aouaoeu/aoeuaoeua')
          expect(path.test_file?).to eq(true)
        end
      end

      context 'when path is to a RSpec file' do
        it 'returns true' do
          path = described_class.new('spec/aouaoeu/aoeuaoeua')
          expect(path.test_file?).to eq(true)
        end
      end
    end

    context 'when path is not to a test file' do
      it 'returns false' do
        path = described_class.new('jackroot/aouaoeu/aoeuaoeua')
        expect(path.test_file?).to eq(false)
      end
    end
  end

  describe '#directory?' do
    context 'when the path is to something on the file system' do
      context 'when the path is to a directory on the file system' do
        it 'returns true' do
          path = described_class.new('resources')
          expect(path.directory?).to eq(true)
        end
      end

      context 'when the path is not to a directory on the file system' do
        it 'returns false' do
          path = described_class.new('resources/demo.gif')
          expect(path.directory?).to eq(false)
        end
      end
    end

    context 'when the path is to something NOT on the file system' do
      it 'returns false' do
        path = described_class.new('jackroot/aouaoeu/aoeuaoeua')
        expect(path.directory?).to eq(false)
      end
    end
  end
end
