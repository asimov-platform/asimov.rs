# See: https://docs.ruby-lang.org/en/4.0/Gem/Specification.html

require 'distrib/ruby/gemspec'

Distrib::Ruby::Gemspec.build!(__FILE__) do |gemspec|
  gemspec.name        = 'asimov.rb'
  gemspec.summary     = "ASIMOV Software Development Kit (SDK) for Ruby"
  gemspec.description = "A polyglot development platform for trustworthy neurosymbolic machine intelligence."
  gemspec.homepage    = "https://asimov.sh"
  gemspec.metadata    = {
    :source_code_uri  => "https://github.com/asimov-platform/asimov-sdk",
    :bug_tracker_uri  => "https://github.com/asimov-platform/asimov-sdk/issues",
    :changelog_uri    => "https://github.com/asimov-platform/asimov-sdk/blob/master/ruby/CHANGES.md",
  }.transform_keys(&:to_s)
end
