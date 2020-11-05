package main

type Item struct {
	name            string
	sellIn, quality int
}

func UpdateQuality(items []*Item) {
	for i := 0; i < len(items); i++ {
		if items[i].quality < 50 {
			if items[i].name == "Aged Brie" {
				items[i].quality = items[i].quality + 1
			} else if items[i].name == "Backstage passes to a TAFKAL80ETC concert" {
				if items[i].sellIn < 6 {
					items[i].quality = items[i].quality + 3
				} else if items[i].sellIn < 11 {
					items[i].quality = items[i].quality + 2
				} else {
					items[i].quality = items[i].quality + 1
				}
			} else if items[i].quality > 0 {
				if items[i].name != "Sulfuras, Hand of Ragnaros" {
					items[i].quality = items[i].quality - 1
				}
			}
		}

		if items[i].name != "Sulfuras, Hand of Ragnaros" {
			items[i].sellIn = items[i].sellIn - 1
		}

		if items[i].sellIn < 0 && items[i].quality > 0 {
			if items[i].name == "Aged Brie" && items[i].quality < 50 {
				items[i].quality = items[i].quality + 1
			} else if items[i].name == "Backstage passes to a TAFKAL80ETC concert" {
				items[i].quality = 0
			} else if items[i].name != "Sulfuras, Hand of Ragnaros" {
				items[i].quality = items[i].quality - 1
			}
		}

	}

}
