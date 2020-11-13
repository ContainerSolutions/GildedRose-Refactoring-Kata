package main

import "testing"

func Test_UpdateQuality(t *testing.T) {

	scenarios := []struct {
		Input, Result *Item
		Days          int
	}{
		{
			Input:  &Item{"apple", 10, 10},
			Days:   10,
			Result: &Item{"apple", 0, 0},
		},
		{
			Input:  &Item{"apple", 5, 40},
			Days:   10,
			Result: &Item{"apple", -5, 25},
		},
		{
			Input:  &Item{"Conjured apple", 50, 40},
			Days:   5,
			Result: &Item{"Conjured apple", 45, 30},
		},
		{
			Input:  &Item{"Conjured apple", 5, 40},
			Days:   7,
			Result: &Item{"Conjured apple", -2, 22},
		},
		{
			Input:  &Item{"Sulfuras, apple", 50, 80},
			Days:   5,
			Result: &Item{"Sulfuras, apple", 50, 80},
		},
		{
			Input:  &Item{"Aged Brie apple", 50, 40},
			Days:   100,
			Result: &Item{"Aged Brie apple", -50, 50},
		},
		{
			Input:  &Item{"Backstage passes apple", 15, 20},
			Days:   7,
			Result: &Item{"Backstage passes apple", 8, 29},
		},
		{
			Input:  &Item{"Backstage passes apple", 17, 10},
			Days:   13,
			Result: &Item{"Backstage passes apple", 4, 30},
		},
		{
			Input:  &Item{"Backstage passes apple", 10, 10},
			Days:   10,
			Result: &Item{"Backstage passes apple", 0, 35},
		},
		{
			Input:  &Item{"Backstage passes apple", 2, 10},
			Days:   3,
			Result: &Item{"Backstage passes apple", -1, 0},
		},
		{
			Input:  &Item{"Backstage passes apple", 2, 10},
			Days:   5,
			Result: &Item{"Backstage passes apple", -3, 0},
		},
		{
			Input:  &Item{"Backstage passes apple", -1, 0},
			Days:   3,
			Result: &Item{"Backstage passes apple", -4, 0},
		},
	}

	for _, s := range scenarios {

		for i := 0; i < s.Days; i++ {
			UpdateQuality([]*Item{s.Input})
		}

		if s.Input.sellIn != s.Result.sellIn || s.Input.quality != s.Result.quality {
			t.Errorf("Product: %s  Expected sellIn: %d != %d ; quality: %d != %d", s.Input.name, s.Input.sellIn, s.Result.sellIn, s.Input.quality, s.Result.quality)
		}
	}

}
