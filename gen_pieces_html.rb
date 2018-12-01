#!/usr/bin/env ruby

PIECES = %w[king queen rook bishop knight pawn]

PIECES.size.times do |n|
  PIECES.combination(n + 1).each do |pieces|
    %w[white black].each do |color|
      classes = (["piece-#{color}"] + pieces).join(" ")
      # classes = "piece-#{color} " + pieces.map{|p| ".#{p}"}.join(" ")
      # line = "td#{classes}{background-image:#{urls};}"
      line = "<td class='#{classes}' />"
      puts line
    end
  end
end
