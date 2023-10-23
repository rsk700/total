use std::collections::HashMap;
use std::fmt;

fn get_args_map(args: &[String], applied_args: &[ValueType]) -> HashMap<String, ValueType> {
    let mut args_map = HashMap::new();
    for i in 0..args.len() {
        args_map.insert(args[i].clone(), applied_args[i].clone());
    }
    args_map
}

#[derive(Debug, Clone)]
pub struct ValueType {
    pub name: String,
    pub args: Vec<ValueType>,
}

impl ValueType {
    pub fn new(name: String, args: Vec<Self>) -> Self {
        Self { name, args }
    }

    pub fn apply_args(&self, applied_args: &[Self]) -> Self {
        let arg_names = self.args.iter().map(|a| a.name.clone()).collect::<Vec<_>>();
        let args_map = get_args_map(&arg_names, applied_args);
        self.apply_args_map(&args_map)
    }

    pub fn apply_args_map(&self, args_map: &HashMap<String, Self>) -> Self {
        if let Some(t) = args_map.get(&self.name) {
            t.clone()
        } else {
            let mapped_args = self
                .args
                .iter()
                .map(|a| a.apply_args_map(args_map))
                .collect();
            ValueType::new(self.name.clone(), mapped_args)
        }
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.args.is_empty() {
            write!(f, "{}", self.name)
        } else {
            let args_string = self
                .args
                .iter()
                .map(|a| a.name.clone())
                .collect::<Vec<_>>()
                .join(", ");
            write!(f, "{}[{}]", self.name, args_string)
        }
    }
}

#[derive(Debug, Clone)]
pub struct LeapStruct {
    pub name: String,
    pub args: Vec<String>,
    pub props: Vec<Property>,
}

impl LeapStruct {
    pub fn new(name: String, args: Vec<String>, props: Vec<Property>) -> Self {
        Self { name, args, props }
    }

    pub fn apply_args(&self, applied_args: &[ValueType]) -> Self {
        let args_map = get_args_map(&self.args, applied_args);
        let new_props = self
            .props
            .iter()
            .map(|p| p.apply_args_map(&args_map))
            .collect();
        Self::new(self.name.clone(), vec![], new_props)
    }
}

#[derive(Debug, Clone)]
pub struct LeapEnum {
    pub name: String,
    pub args: Vec<String>,
    pub variants: Vec<Property>,
}

impl LeapEnum {
    pub fn new(name: String, args: Vec<String>, variants: Vec<Property>) -> Self {
        Self {
            name,
            args,
            variants,
        }
    }

    pub fn apply_args(&self, applied_args: &[ValueType]) -> Self {
        let args_map = get_args_map(&self.args, applied_args);
        let new_variants = self
            .variants
            .iter()
            .map(|v| v.apply_args_map(&args_map))
            .collect();
        Self::new(self.name.clone(), vec![], new_variants)
    }
}

#[derive(Debug, Clone)]
pub struct LeapVariant {
    pub enum_name: String,
    pub variant: Property,
    pub args: Vec<String>,
    pub props: Vec<Property>,
}

impl LeapVariant {
    pub fn new(
        enum_name: String,
        variant: Property,
        args: Vec<String>,
        props: Vec<Property>,
    ) -> Self {
        Self {
            enum_name,
            variant,
            args,
            props,
        }
    }

    pub fn apply_args(&self, applied_args: &[ValueType]) -> Self {
        let args_map = get_args_map(&self.args, applied_args);
        let new_props = self
            .props
            .iter()
            .map(|p| p.apply_args_map(&args_map))
            .collect();
        Self::new(
            self.enum_name.clone(),
            self.variant.apply_args_map(&args_map),
            vec![],
            new_props,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub prop_type: ValueType,
}

impl Property {
    pub fn new(name: String, prop_type: ValueType) -> Self {
        Self { name, prop_type }
    }

    pub fn apply_args_map(&self, args_map: &HashMap<String, ValueType>) -> Property {
        Self::new(self.name.clone(), self.prop_type.apply_args_map(args_map))
    }
}
