export default function (sequelize, DataTypes) {
	return sequelize.define(
		'Event',
		{
			id: { type: DataTypes.BIGINT, primaryKey: true },
			name: { type: DataTypes.TEXT, allowNull: false },
			guild_id: { type: DataTypes.BIGINT, allowNull: false },
			owner_id: { type: DataTypes.BIGINT, allowNull: true }
		},
		{
			sequelize,
			tableName: 'event',
			timestamps: false
		}
	);
}
