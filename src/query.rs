use adapter::Adapter;
use attribute::Attribute;
use model::{Model, RecordSet};

pub enum SortOrder {
    ASC,
    DESC,
}

pub enum Filter<'a> {
    IsNull(&'a str),
    IsNotNull(&'a str),

    Equal(&'a str, &'a Attribute),
    In(&'a str, Vec<&'a Attribute>),

    And(Box<Filter<'a>>, Box<Filter<'a>>),
    Or(Box<Filter<'a>>, Box<Filter<'a>>),
}

pub struct Query<'a> {
    pub model: Model,
    pub include: Option<Vec<Model>>,
    pub fields: Option<Vec<&'a str>>,
    pub sort: Option<Vec<(&'a str, SortOrder)>>,
    pub filter: Option<Filter<'a>>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

impl<'a> Query<'a> {
    pub fn new(model: Model) -> Query<'a> {
        Query {
            model: model,
            include: None,
            fields: None,
            sort: None,
            filter: None,
            offset: None,
            limit: None,
        }
    }

    pub fn get<A: Adapter>(self, adapter: &A) -> Option<RecordSet> {
        adapter.query(&self)
    }

    pub fn include(mut self, models: Vec<Model>) -> Query<'a> {
        self.include = Some(models);
        self
    }

    pub fn select(mut self, fileds: Vec<&'a str>) -> Query<'a> {
        self.fields = Some(fileds);
        self
    }

    pub fn order_by(mut self, name: &'a str, order: SortOrder) -> Query<'a> {
        let mut sort = if let Some(sort) = self.sort {
            sort
        } else {
            vec![]
        };
        sort.push((name, order));
        self.sort = Some(sort);
        self
    }

    pub fn offset(mut self, offset: i32) -> Query<'a> {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: i32) -> Query<'a> {
        self.limit = Some(limit);
        self
    }

    pub fn where_(self, name: &'a str) -> WhereFilterBuilder<'a> {
        WhereFilterBuilder {
            query: self,
            name: name,
        }
    }

    pub fn and(self, name: &'a str) -> AndFilterBuilder<'a> {
        AndFilterBuilder {
            query: self,
            name: name,
        }
    }

    pub fn or(self, name: &'a str) -> OrFilterBuilder<'a> {
        OrFilterBuilder {
            query: self,
            name: name,
        }
    }
}

pub struct WhereFilterBuilder<'a> {
    query: Query<'a>,
    name: &'a str,
}

impl<'a> WhereFilterBuilder<'a> {
    pub fn is_null(mut self) -> Query<'a> {
        self.query.filter = Some(Filter::IsNull(self.name));
        self.query
    }

    pub fn is_not_null(mut self) -> Query<'a> {
        self.query.filter = Some(Filter::IsNotNull(self.name));
        self.query
    }

    pub fn eq(mut self, attribute: &'a Attribute) -> Query<'a> {
        self.query.filter = Some(Filter::Equal(self.name, attribute));
        self.query
    }

    pub fn in_(mut self, attributes: Vec<&'a Attribute>) -> Query<'a> {
        self.query.filter = Some(Filter::In(self.name, attributes));
        self.query
    }
}

pub struct AndFilterBuilder<'a> {
    query: Query<'a>,
    name: &'a str,
}

impl<'a> AndFilterBuilder<'a> {
    pub fn is_null(mut self) -> Query<'a> {
        self.query.filter = Some(Filter::And(Box::new(self.query.filter.unwrap()),
                                             Box::new(Filter::IsNull(self.name))));
        self.query
    }

    pub fn is_not_null(mut self) -> Query<'a> {
        self.query.filter = Some(Filter::And(Box::new(self.query.filter.unwrap()),
                                             Box::new(Filter::IsNotNull(self.name))));
        self.query
    }

    pub fn eq(mut self, attribute: &'a Attribute) -> Query<'a> {
        self.query.filter = Some(Filter::And(Box::new(self.query.filter.unwrap()),
                                             Box::new(Filter::Equal(self.name, attribute))));
        self.query
    }

    pub fn in_(mut self, attributes: Vec<&'a Attribute>) -> Query<'a> {
        self.query.filter = Some(Filter::And(Box::new(self.query.filter.unwrap()),
                                             Box::new(Filter::In(self.name, attributes))));
        self.query
    }
}

pub struct OrFilterBuilder<'a> {
    query: Query<'a>,
    name: &'a str,
}

impl<'a> OrFilterBuilder<'a> {
    pub fn is_null(mut self) -> Query<'a> {
        self.query.filter = Some(Filter::Or(Box::new(self.query.filter.unwrap()),
                                            Box::new(Filter::IsNull(self.name))));
        self.query
    }

    pub fn is_not_null(mut self) -> Query<'a> {
        self.query.filter = Some(Filter::Or(Box::new(self.query.filter.unwrap()),
                                            Box::new(Filter::IsNotNull(self.name))));
        self.query
    }

    pub fn eq(mut self, attribute: &'a Attribute) -> Query<'a> {
        self.query.filter = Some(Filter::Or(Box::new(self.query.filter.unwrap()),
                                            Box::new(Filter::Equal(self.name, attribute))));
        self.query
    }

    pub fn in_(mut self, attributes: Vec<&'a Attribute>) -> Query<'a> {
        self.query.filter = Some(Filter::Or(Box::new(self.query.filter.unwrap()),
                                            Box::new(Filter::In(self.name, attributes))));
        self.query
    }
}
