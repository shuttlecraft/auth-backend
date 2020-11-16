/*
* Copyright (C) 2020  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

table! {
    users (user_id) {
        user_id -> Int4,
        username -> Varchar,
        user_uuid -> Uuid,
        hash -> Bytea,
        salt -> Varchar,
        email -> Nullable<Varchar>,
        role -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
    }
}
