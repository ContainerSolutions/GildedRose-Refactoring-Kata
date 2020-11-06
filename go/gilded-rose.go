package main

import (
	"strings"
)

type Item struct {
	name            string
	sellIn, quality int
}

func UpdateQuality(items []*Item) {
	for i := 0; i < len(items); i++ {

		if items[i].name == "Aged Brie" {
			IncreaseQualityByOne(items[i])
			if items[i].sellIn <= 0 {
				IncreaseQualityByOne(items[i])
			}
		} else if items[i].name == "Backstage passes to a TAFKAL80ETC concert" {
			if items[i].sellIn > 0 {
				IncreaseQualityByOne(items[i])
				if items[i].sellIn < 6 {
					IncreaseQualityByOne(items[i])
				}
				if items[i].sellIn < 11 {
					IncreaseQualityByOne(items[i])
				}

			} else {
				items[i].quality = 0
			}

		} else if strings.Contains(items[i].name, "Conjured") {
			if items[i].quality > 0 {
				items[i].quality = items[i].quality - 2
			}
		} else if items[i].name != "Sulfuras, Hand of Ragnaros" {
			if items[i].sellIn > 0 {
				items[i].quality = items[i].quality - 1
			} else {
				items[i].quality = 0
			}
		}

		if items[i].name != "Sulfuras, Hand of Ragnaros" {
			items[i].sellIn = items[i].sellIn - 1
		}

	}

}

func IncreaseQualityByOne(item *Item) {
	if item.quality < 50 {
		item.quality = item.quality + 1
	}
}
