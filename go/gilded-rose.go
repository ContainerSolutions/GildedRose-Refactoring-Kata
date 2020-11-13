package main

import "strings"

type Item struct {
	name            string
	sellIn, quality int
}

func UpdateQuality(items []*Item) {
	for _, item := range items {
		qualityChange := defineQualityChange(item)
		if isException(item.name) {
			continue
		}
		item.sellIn--
		item.quality = item.quality + qualityChange
		if item.quality < 0 {
			item.quality = 0
		}
		if item.quality > 50 {
			item.quality = 50
		}
	}
}

func isException(name string) bool {
	return strings.HasPrefix(name, "Sulfuras")
}

func defineQualityChange(item *Item) (qualityChange int) {
	switch n := item.name; {
	case strings.HasPrefix(n, "Aged Brie"):
		qualityChange = 1
	case strings.HasPrefix(n, "Backstage passes"):
		if item.sellIn <= 0 {
			qualityChange = -item.quality
			break
		}
		if item.sellIn <= 5 {
			qualityChange = 3
			break
		}
		if item.sellIn <= 10 {
			qualityChange = 2
			break
		}
		qualityChange = 1
	case strings.HasPrefix(n, "Conjured"):
		if item.sellIn > 0 {
			qualityChange = -2
		} else {
			qualityChange = -4
		}
	default:
		if item.sellIn > 0 {
			qualityChange = -1
		} else {
			qualityChange = -2
		}
	}
	return qualityChange
}
