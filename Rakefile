require 'fileutils'
require 'json'
require 'pathname'
require 'stringio'
require 'tomlrb' # https://rubygems.org/gems/tomlrb
require 'yaml'

task default: %w(.cargo/packages.json .cargo/packages.md)

PACKAGES = Dir['lib/**/Cargo.toml'].sort_by do |path|
  path.delete_prefix('lib/').delete_suffix('/Cargo.toml')
end.map { Pathname(it) }.freeze

file '.cargo/packages.json': PACKAGES do |t|
  File.open(t.name, 'w') do |out|
    out.puts generate_json(t.prerequisites)
  end
end

file '.cargo/packages.md': PACKAGES do |t|
  File.open(t.name, 'w') do |out|
    out.puts generate_markdown(t.prerequisites)
  end
end

namespace :version do
  desc "Bump the version number"
  task :bump do
    old_version = File.read('VERSION').strip
    new_version = old_version.gsub(/\.\d+$/, &:succ)
    warn `git grep -l #{old_version} | xargs sd -F #{old_version} #{new_version}`.chomp
  end
end

def generate_markdown(input_paths)
  StringIO.open do |out|
    out.puts "| Package | Crate | Docs |"
    out.puts "| :------ | :---- | :--- |"
    load_projects(input_paths).each do |project|
      package_name = project[:package][:name]
      package_link = "[#{package_name}](https://github.com/asimov-platform/asimov.rs/tree/master/lib/#{package_name})"
      #package_summary = project[:package][:description]
      package_links = [
        "[![Package](https://img.shields.io/crates/v/#{package_name})](https://crates.io/crates/#{package_name})",
        "[![Documentation](https://img.shields.io/docsrs/#{package_name}?label=docs.rs)](https://docs.rs/#{package_name})",
      ]
      out.puts "| " + [
        package_link,
        #package_summary,
        package_links[0],
        package_links[1],
      ].join(" | ") + " |"
    end
    out.string
  end
end

def generate_json(input_paths)
  JSON.pretty_unparse(load_projects(input_paths))
end

def load_projects(input_paths)
  input_paths.map do |input_path|
    manifest = Tomlrb.load_file(input_path, symbolize_keys: true)
    remove_workspace_only(manifest)
  end
end

def remove_workspace_only(data, in_deps: false)
  case data
    when Hash
      data.reject { |_, v| !in_deps && (v == {workspace: true}) }
          .to_h { |k, v| [k, remove_workspace_only(v, in_deps: k.to_s.include?("dependencies"))] }
    when Array
      data.map { |item| remove_workspace_only(item, in_deps: in_deps) }
    else data
  end
end
