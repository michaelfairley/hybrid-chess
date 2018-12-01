#!/usr/bin/env ruby

PIECES = %w[king queen rook bishop knight pawn]

PIECES.size.times do |n|
  PIECES.combination(n + 1).each do |pieces|
    %w[white black].each do |color|
      classes = ".piece-#{color}" + pieces.map{|p| ".#{p}"}.join("")
      urls = pieces.map{|piece| "url(\"images/#{color}_#{piece}.svg\")" }.join(",")
      line = "td#{classes}{background-image:#{urls};}"
      puts line
    end
  end
end
