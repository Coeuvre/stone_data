use attribute::{Attribute, Attributes};

pub enum SortOrder {
    ASC,
    DESC,
}

pub enum Filter<'a> {
    IsNull(&'a str),

    Equal(&'a str, &'a Attribute),
    In(&'a str, Vec<&'a Attribute>),

    And(Box<Filter<'a>>, Box<Filter<'a>>),
    Or(Box<Filter<'a>>, Box<Filter<'a>>),
}

pub struct Query<'a> {
    pub table: Option<&'a str>,
    pub fields: Option<Vec<&'a str>>,
    pub sort: Option<Vec<(&'a str, SortOrder)>>,
    pub filter: Option<Filter<'a>>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

impl<'a> Query<'a> {
    pub fn table(table: &'a str) -> Query<'a> {
        Query {
            table: Some(table),
            fields: None,
            sort: None,
            filter: None,
            offset: None,
            limit: None,
        }
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

    pub fn filter(self, name: &'a str) -> WhereFilterBuilder<'a> {
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

    pub fn eq(mut self, attribute: &'a Attribute) -> Query<'a> {
        self.query.filter = Some(Filter::Equal(self.name, attribute));
        self.query
    }

    pub fn en(mut self, attributes: Vec<&'a Attribute>) -> Query<'a> {
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

    pub fn eq(mut self, attribute: &'a Attribute) -> Query<'a> {
        self.query.filter = Some(Filter::And(Box::new(self.query.filter.unwrap()),
                                             Box::new(Filter::Equal(self.name, attribute))));
        self.query
    }

    pub fn en(mut self, attributes: Vec<&'a Attribute>) -> Query<'a> {
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

    pub fn eq(mut self, attribute: &'a Attribute) -> Query<'a> {
        self.query.filter = Some(Filter::Or(Box::new(self.query.filter.unwrap()),
                                            Box::new(Filter::Equal(self.name, attribute))));
        self.query
    }

    pub fn en(mut self, attributes: Vec<&'a Attribute>) -> Query<'a> {
        self.query.filter = Some(Filter::Or(Box::new(self.query.filter.unwrap()),
                                            Box::new(Filter::In(self.name, attributes))));
        self.query
    }
}

pub struct QueryResult {
    pub attributes: Option<Vec<Attributes>>,
    pub page: i32,
    pub count: i32,
}

#[test]
fn test() {
    let id = 1.into();
    let name = "coeuvre".to_string().into();
    Query::table("user").select(vec!["id", "name"]).filter("id").eq(&id).and("name").eq(&name).order_by("id", SortOrder::ASC).limit(1);
}
