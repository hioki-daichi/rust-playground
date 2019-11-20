require 'bundler/inline'

gemfile do
  source 'https://rubygems.org'
  gem 'dataloader'
  gem 'sqlite3'
  gem 'activerecord', require: 'active_record'
end

ActiveRecord::Base.establish_connection(adapter: 'sqlite3', database: ':memory:')

Class.new(ActiveRecord::Migration[5.2]) do
  def self.up
    create_table :foos do |t|
      t.string :foo
    end

    create_table :bars do |t|
      t.bigint :foo_id
      t.string :bar
    end
  end
end.migrate(:up)

class Foo < ActiveRecord::Base
  has_many :bars
end

class Bar < ActiveRecord::Base
  belongs_to :foo
end

%w[foo bar baz qux].each do |foo|
  Foo.create(foo: foo)
end

foo_loader = Dataloader.new do |ids|
  Foo.find(*ids)
end

# bar_loader = Dataloader.new do |foo_id|
#   Bar.find_by(foo_id: foo_id)
# end

ActiveRecord::Base.logger = Logger.new(STDOUT)

promise_one = foo_loader.load(1)
promise_two = foo_loader.load_many([2, 3])

foo1 = promise_one.sync
foo2, foo3 = promise_two.sync

p Bar.all

# promise_three = bar_loader.load_many([foo1, foo2, foo3].map(&:id))
#
# bar1, bar2, bar3 = promise_three.sync
# p bar1
# p bar2
# p bar3
