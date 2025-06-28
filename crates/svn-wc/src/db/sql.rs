//! `wc-queries.sql`

use rusqlite::Connection;
use svn_types::Depth;

use super::DBError;
use super::WcRoot;

impl WcRoot {
    /// Get a reference to the SQLite connection for this working copy root.
    #[inline]
    fn conn(&self) -> &Connection {
        &self.sdb.conn
    }

    #[inline]
    fn wc_id(&self) -> i64 {
        // self.sdb.wc_id
        todo!()
    }

    #[inline]
    fn wc_id_str(&self) -> String {
        self.wc_id().to_string()
    }

    /// STMT_SELECT_NODE_INFO
    pub fn node_info(&self, local_relpath: &str) -> Result<Vec<NodeInfo>, DBError> {
        const STMT: &str = r#"
            SELECT op_depth, repos_id, repos_path, presence, kind, revision, checksum,
              translated_size, changed_revision, changed_date, changed_author, depth,
              symlink_target, last_mod_time, properties, moved_here, inherited_props,
              moved_to
            FROM nodes
            WHERE wc_id = :wc_id AND local_relpath = :local_relpath
            ORDER BY op_depth DESC
            "#;
        let mut stmt = self.conn().prepare(STMT)?;
        let wc_id_str = self.wc_id_str();
        let wc_id = wc_id_str.as_str();
        let info_iter = stmt.query_map(
            &[(":wc_id", &wc_id), (":local_relpath", &local_relpath)],
            |row| {
                // Extract the necessary fields from the row and create a NodeInfo instance
                // This is a placeholder; actual implementation will depend on NodeInfo structure
                Ok(NodeInfo {
                    op_depth: row.get(0)?,
                })
            },
        )?;
        let mut ret = vec![];
        for info in info_iter {
            ret.push(info?);
        }
        Ok(ret)
    }

    /// STMT_SELECT_ANCESTOR_WCLOCKS
    pub fn ancestor_wclocks(&self) -> Result<AncestorWclocksResult, DBError> {
        let stmt = r#"
            SELECT local_dir_relpath, locked_levels
            FROM wc_lock
            WHERE wc_id = ?
            "#;
        self.conn()
            .query_row(stmt, [self.wc_id()], |row| {
                Ok(AncestorWclocksResult {
                    local_dir_relpath: row.get(0)?,
                    locked_levels: row.get(1)?,
                })
            })
            .map_err(|e| DBError::from(e))
    }

    /// `STMT_CLEAR_ACTUAL_NODE_LEAVING_CONFLICT`
    pub fn clear_actual_node_leaving_conflict(&self, local_relpath: &str) -> Result<(), DBError> {
        const STMT: &str = r#"
        UPDATE actual_node
        SET properties = NULL,
            text_mod = NULL,
            tree_conflict_data = NULL,
            older_checksum = NULL,
            left_checksum = NULL,
            right_checksum = NULL,
            changelist = NULL
        WHERE wc_id = ?1 AND local_relpath = ?2
            "#;
        self.conn()
            .execute(STMT, &[&self.wc_id_str(), local_relpath])
            .map_err(|e| DBError::from(e))?;
        Ok(())
    }

    /// `STMT_DELETE_ACTUAL_NODE`
    pub fn delete_actual_node(&self, local_relpath: &str) -> Result<(), DBError> {
        const STMT: &str = r#"
        DELETE FROM actual_node
        WHERE wc_id = ?1 AND local_relpath = ?2
            "#;
        self.conn()
            .execute(STMT, &[&self.wc_id_str(), local_relpath])
            .map_err(|e| DBError::from(e))?;
        Ok(())
    }

    /// `STMT_UPDATE_NODE_BASE_DEPTH`
    pub fn update_node_base_depth(&self, local_relpath: &str, depth: Depth) -> Result<(), DBError> {
        const STMT: &str = r#"
        UPDATE nodes SET depth = ?3
        WHERE wc_id = ?1 AND local_relpath = ?2 AND op_depth = 0
          AND kind=MAP_DIR
          AND presence IN (MAP_NORMAL, MAP_INCOMPLETE)
          "#;
        self.conn()
            .execute(STMT, &[&self.wc_id_str(), local_relpath, &depth.as_str()])
            .map_err(|e| DBError::from(e))?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct NodeInfo {
    op_depth: i32,
}

#[derive(Debug, Clone)]
pub struct AncestorWclocksResult {
    local_dir_relpath: String,
    locked_levels: i64,
}
