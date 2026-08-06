# This is free and unencumbered software released into the public domain.

module ASIMOV; end
module ASIMOV::VERSION; end

module ASIMOV::VERSION
  FILE = File.expand_path('../../../VERSION', __FILE__)
  STRING = File.read(FILE).chomp.freeze
  MAJOR, MINOR, PATCH, EXTRA = STRING.split('.').map(&:freeze)
end # ASIMOV::VERSION
