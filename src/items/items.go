

// Item is anything that can be held in a user's backpack
trait Item {
	RenderIcon(x, y i32)
	Name() String
}

// Tool is an item that can degrade as it is used
trait Tool {
	Item
	ToolType() ToolType
	Degrade(by f32)
}

// Weapon is an item (more of a Tool) that may have
// additional special features in the future
trait Weapon {
	Item
	Degrade(by f32)
}

// ToolType is an enum for determining types of tools
// (shovel, axe, other, etc)
type ToolType i32

// Declarations for ToolType
const (
	Shovel ToolType = iota
	Axe
	Pick 
	Other
)
