package main

import (
	"testing"
)

func LoopDays(items []*Item, days int) {
	for day := 0; day < days; day++ {
		UpdateQuality(items)
	}
}

func TestSulfuras(t *testing.T) {

	var items = []*Item{
		&Item{"Sulfuras, Hand of Ragnaros", 0, 80},
	}

	days := 55

	LoopDays(items, days)

	if items[0].quality != 80 {
		t.Errorf("Quality: Expected %s but got %d ", "80", items[0].quality)
	}

	if items[0].sellIn != 0 {
		t.Errorf("Quality: Expected %s but got %d ", "80", items[0].quality)
	}
}

func TestAgedBrie(t *testing.T) {

	var items = []*Item{
		&Item{"Aged Brie", 2, 0},
	}

	days := 2

	LoopDays(items, days)

	if items[0].quality != 2 {
		t.Errorf("Quality: Expected less than 2, but got %d ", items[0].quality)
	}

	days = 1

	LoopDays(items, days)

	if items[0].quality != 4 {
		t.Errorf("Quality: Expected less than 4, but got %d ", items[0].quality)
	}

	days = 55

	LoopDays(items, days)

	if items[0].quality != 50 {
		t.Errorf("Quality: Expected less than 50, but got %d ", items[0].quality)
	}

}

func TestBackstagePasses(t *testing.T) {

	var items = []*Item{
		&Item{"Backstage passes to a TAFKAL80ETC concert", 15, 20},
	}

	days := 5

	LoopDays(items, days)

	if items[0].quality != 25 {
		t.Errorf("Quality: Expected 25, but got %d ", items[0].quality)
	}

	days = 5

	LoopDays(items, days)

	if items[0].quality != 35 {
		t.Errorf("Quality: Expected 35, but got %d ", items[0].quality)
	}

	days = 5

	LoopDays(items, days)

	if items[0].quality != 50 {
		t.Errorf("Quality: Expected 50, but got %d ", items[0].quality)
	}

	days = 5

	LoopDays(items, days)

	if items[0].quality != 0 {
		t.Errorf("Quality: Expected 0, but got %d ", items[0].quality)
	}

}

func TestConjuredManaCake(t *testing.T) {

	var items = []*Item{
		&Item{"Conjured Mana Cake", 3, 6},
	}

	days := 3

	LoopDays(items, days)

	if items[0].quality != 3 {
		t.Errorf("Quality: Expected 3, but got %d ", items[0].quality)
	}

	days = 3

	LoopDays(items, days)

	if items[0].quality != 0 {
		t.Errorf("Quality: Expected 0, but got %d ", items[0].quality)
	}

	days = 3

	LoopDays(items, days)

	if items[0].quality != 0 {
		t.Errorf("Quality: Expected -3, but got %d ", items[0].quality)
	}

}
